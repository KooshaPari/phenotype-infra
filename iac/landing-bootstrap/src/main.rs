//! phenotype-landing-bootstrap
//!
//! End-to-end automation for Tier 2 of the Phenotype org-pages tree.
//! Given a slug + GitHub repo, this tool:
//!   1. Checks GitHub `homepageUrl` for a `.dev` collision (skip if present + emit
//!      a redirect-only marker for downstream Vercel config).
//!   2. Creates a Cloudflare CNAME `<slug> → cname.vercel-dns.com`.
//!   3. Scaffolds a fresh `<slug>-landing/` directory from the
//!      `agileplus-landing` template, swapping in the slug-specific
//!      title/description/REPO constant.
//!   4. `git init`, commits, creates `KooshaPari/<slug>-landing` on GitHub, pushes.
//!   5. `vercel link --yes` and `vercel deploy --prod --yes` against the new
//!      project.
//!   6. Attaches `<slug>.kooshapari.com` as a custom domain.
//!
//! Idempotent: each step is a no-op if already done (404 → create, 4xx existing → skip).
//!
//! Usage:
//!   phenotype-landing-bootstrap \
//!     --slug thegent \
//!     --repo KooshaPari/thegent \
//!     --title "thegent" \
//!     --tagline "Python agent runtime with tool registry"
//!
//! Env required:
//!   - CF_API_TOKEN (or --cf-token-file ~/.cloudflare-token)
//!   - CF_ZONE_ID (default: 6c9edab581e9c7b8fdb6a83adc6878ea = kooshapari.com)
//!   - GITHUB_TOKEN (via `gh auth token`)
//!
//! Wraps: `gh` CLI + `vercel` CLI + Cloudflare REST API.

use anyhow::{Context, Result, bail};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

const ZONE_ID_DEFAULT: &str = "6c9edab581e9c7b8fdb6a83adc6878ea";

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// DNS slug (e.g. "thegent" → thegent.kooshapari.com).
    #[arg(long)]
    slug: String,

    /// Source repo, e.g. "KooshaPari/thegent".
    #[arg(long)]
    repo: String,

    /// Display title (falls back to slug capitalized).
    #[arg(long)]
    title: Option<String>,

    /// Tagline / meta description (falls back to GitHub description).
    #[arg(long)]
    tagline: Option<String>,

    /// Template repo path (default: ../../../agileplus-landing).
    #[arg(long, default_value = "../../../agileplus-landing")]
    template: PathBuf,

    /// Output dir for the new landing repo (default: ../../../<slug>-landing).
    #[arg(long)]
    out: Option<PathBuf>,

    /// Cloudflare zone id.
    #[arg(long, env = "CF_ZONE_ID", default_value = ZONE_ID_DEFAULT)]
    cf_zone_id: String,

    /// Cloudflare API token file (overrides CF_API_TOKEN env).
    #[arg(long, default_value = "~/.cloudflare-token")]
    cf_token_file: String,

    /// Skip Vercel deploy (useful for dry-run).
    #[arg(long)]
    skip_vercel: bool,

    /// Skip GitHub repo creation/push.
    #[arg(long)]
    skip_github: bool,

    /// Print actions only, do not execute.
    #[arg(long)]
    dry_run: bool,

    /// Skip Tier-3 path microfrontends (`/docs`, `/qa`, `/otel`, `/preview/<pr#>`).
    /// By default Tier-3 surfaces are scaffolded from the template repo. Use this
    /// flag to scaffold a minimal Tier-2 landing only (just the homepage).
    /// See `docs/governance/path-microfrontends-tier3.md`.
    #[arg(long)]
    minimal: bool,
}

/// Subdirectories under `src/pages/` that constitute Tier-3 microfrontends.
/// Skipped when `--minimal` is passed.
const TIER3_PAGE_DIRS: &[&str] = &["src/pages/docs", "src/pages/qa", "src/pages/otel", "src/pages/preview"];

#[derive(Debug, Deserialize)]
struct GhRepoMeta {
    description: Option<String>,
    #[serde(rename = "homepageUrl")]
    homepage_url: Option<String>,
}

#[derive(Debug, Serialize)]
struct CfCnameBody<'a> {
    r#type: &'a str,
    name: &'a str,
    content: &'a str,
    proxied: bool,
    ttl: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let slug = args.slug.as_str();
    let domain = format!("{slug}.kooshapari.com");
    let title = args.title.clone().unwrap_or_else(|| capitalize(slug));
    let out_dir = args
        .out
        .clone()
        .unwrap_or_else(|| PathBuf::from(format!("../../../{slug}-landing")));

    eprintln!("→ bootstrap {domain} from {}", args.repo);

    // Step 1: Check .dev coexistence.
    let meta = gh_repo_meta(&args.repo)?;
    let tagline = args
        .tagline
        .clone()
        .or_else(|| meta.description.clone())
        .unwrap_or_else(|| format!("{title} — Phenotype project"));

    let dev_canonical = meta
        .homepage_url
        .as_deref()
        .filter(|u| u.contains(".dev"))
        .map(String::from);
    if let Some(ref canonical) = dev_canonical {
        eprintln!("  ⚠ .dev canonical detected: {canonical}");
        eprintln!("  → will mark for redirect-only deploy (no full landing).");
    }

    // Step 2: Cloudflare DNS CNAME.
    let cf_token = read_cf_token(&args.cf_token_file)?;
    cf_upsert_cname(&cf_token, &args.cf_zone_id, slug, &args.dry_run)?;

    // Step 3: Scaffold from template (or write a redirect-only vercel.json).
    if let Some(canonical) = dev_canonical.as_deref() {
        scaffold_redirect(&out_dir, slug, canonical, &args.dry_run)?;
    } else {
        scaffold_full(
            &args.template,
            &out_dir,
            slug,
            &args.repo,
            &title,
            &tagline,
            &args.dry_run,
            args.minimal,
        )?;
        if args.minimal {
            eprintln!("  ↺ --minimal: skipped Tier-3 microfrontends ({})", TIER3_PAGE_DIRS.join(", "));
        } else {
            eprintln!("  ✓ Tier-3 microfrontends scaffolded: /docs, /qa, /otel, /preview/<pr#>");
            eprintln!("    spec: docs/governance/path-microfrontends-tier3.md");
        }
    }

    // Step 4: GitHub.
    if !args.skip_github {
        let gh_repo = format!("KooshaPari/{slug}-landing");
        git_init_commit_push(&out_dir, &gh_repo, &domain, &args.dry_run)?;
    }

    // Step 5/6: Vercel.
    if !args.skip_vercel {
        vercel_link_deploy_attach(&out_dir, slug, &domain, &args.dry_run)?;
    }

    eprintln!("✓ {domain} bootstrap complete");
    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}

fn read_cf_token(path: &str) -> Result<String> {
    if let Ok(t) = std::env::var("CF_API_TOKEN") {
        if !t.is_empty() {
            return Ok(t);
        }
    }
    let expanded = shellexpand::tilde(path).into_owned();
    std::fs::read_to_string(&expanded)
        .map(|s| s.trim().to_string())
        .with_context(|| format!("read cf token at {expanded}"))
}

fn gh_repo_meta(repo: &str) -> Result<GhRepoMeta> {
    let out = Command::new("gh")
        .args(["repo", "view", repo, "--json", "description,homepageUrl"])
        .output()
        .context("gh repo view")?;
    if !out.status.success() {
        bail!("gh repo view {repo}: {}", String::from_utf8_lossy(&out.stderr));
    }
    Ok(serde_json::from_slice(&out.stdout)?)
}

fn cf_upsert_cname(token: &str, zone: &str, slug: &str, dry: &bool) -> Result<()> {
    if *dry {
        eprintln!("  [dry] CF upsert CNAME {slug} → cname.vercel-dns.com");
        return Ok(());
    }
    let body = CfCnameBody {
        r#type: "CNAME",
        name: slug,
        content: "cname.vercel-dns.com",
        proxied: false,
        ttl: 1,
    };
    let url = format!("https://api.cloudflare.com/client/v4/zones/{zone}/dns_records");
    let resp = ureq::post(&url)
        .set("Authorization", &format!("Bearer {token}"))
        .set("Content-Type", "application/json")
        .send_json(serde_json::to_value(&body)?);
    match resp {
        Ok(r) => {
            let v: serde_json::Value = r.into_json()?;
            let success = v.get("success").and_then(|x| x.as_bool()).unwrap_or(false);
            if success {
                eprintln!("  ✓ CF CNAME {slug} created");
            } else {
                eprintln!("  ⚠ CF response (likely already exists): {v}");
            }
        }
        Err(ureq::Error::Status(_, r)) => {
            let v: serde_json::Value = r.into_json().unwrap_or_default();
            eprintln!("  ⚠ CF non-2xx (assuming already exists): {v}");
        }
        Err(e) => bail!("cf request: {e}"),
    }
    Ok(())
}

fn scaffold_full(
    template: &Path,
    out: &Path,
    slug: &str,
    repo: &str,
    title: &str,
    tagline: &str,
    dry: &bool,
    minimal: bool,
) -> Result<()> {
    if *dry {
        eprintln!("  [dry] scaffold {slug}-landing from {} (minimal={minimal})", template.display());
        return Ok(());
    }
    if out.exists() {
        eprintln!("  ↺ {} exists, skipping scaffold", out.display());
        return Ok(());
    }
    std::fs::create_dir_all(out)?;

    for entry in walkdir::WalkDir::new(template).into_iter().filter_map(|e| e.ok()) {
        let rel = match entry.path().strip_prefix(template) {
            Ok(r) => r,
            Err(_) => continue,
        };
        if rel.as_os_str().is_empty() {
            continue;
        }
        let comps: Vec<_> = rel.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect();
        if comps.iter().any(|c| matches!(c.as_str(), "node_modules" | "dist" | ".astro" | ".vercel" | ".git" | "bun.lockb")) {
            continue;
        }
        // --minimal: skip Tier-3 page subtrees so the scaffold ships only the
        // Tier-2 homepage. The Tier-3 spec lives in
        // `phenotype-infra/docs/governance/path-microfrontends-tier3.md`.
        if minimal {
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            if TIER3_PAGE_DIRS.iter().any(|d| rel_str == *d || rel_str.starts_with(&format!("{d}/"))) {
                continue;
            }
        }
        let dst = out.join(rel);
        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&dst)?;
            continue;
        }
        let bytes = std::fs::read(entry.path())?;
        let text = String::from_utf8(bytes.clone()).ok();
        if let Some(s) = text {
            let replaced = s
                .replace("agileplus-landing", &format!("{slug}-landing"))
                .replace("agileplus.kooshapari.com", &format!("{slug}.kooshapari.com"))
                .replace("KooshaPari/AgilePlus", repo)
                .replace("AgilePlus — Spec-driven development engine", &format!("{title} — {tagline}"))
                .replace("AgilePlus", title);
            std::fs::write(&dst, replaced)?;
        } else {
            std::fs::write(&dst, &bytes)?;
        }
    }
    eprintln!("  ✓ scaffolded {}", out.display());
    Ok(())
}

fn scaffold_redirect(out: &Path, slug: &str, canonical: &str, dry: &bool) -> Result<()> {
    if *dry {
        eprintln!("  [dry] scaffold redirect-only for {slug} → {canonical}");
        return Ok(());
    }
    std::fs::create_dir_all(out)?;
    let vercel_json = serde_json::json!({
        "$schema": "https://openapi.vercel.sh/vercel.json",
        "redirects": [
            { "source": "/(.*)", "destination": format!("{}/$1", canonical.trim_end_matches('/')), "permanent": true }
        ],
        "trailingSlash": false
    });
    std::fs::write(out.join("vercel.json"), serde_json::to_string_pretty(&vercel_json)?)?;
    std::fs::write(
        out.join("README.md"),
        format!("# {slug}-landing\n\n301 redirect from {slug}.kooshapari.com → {canonical} (canonical .dev domain).\n"),
    )?;
    std::fs::write(out.join(".gitignore"), "node_modules/\n.vercel/\n.DS_Store\n")?;
    Ok(())
}

fn run(dir: &Path, prog: &str, args: &[&str], dry: &bool) -> Result<String> {
    if *dry {
        eprintln!("  [dry] $ {prog} {}", args.join(" "));
        return Ok(String::new());
    }
    let out = Command::new(prog).args(args).current_dir(dir).output()
        .with_context(|| format!("run {prog} {:?}", args))?;
    if !out.status.success() {
        bail!("{prog} {:?} failed: {}", args, String::from_utf8_lossy(&out.stderr));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn git_init_commit_push(out: &Path, gh_repo: &str, domain: &str, dry: &bool) -> Result<()> {
    if !out.join(".git").exists() {
        run(out, "git", &["init", "-b", "main"], dry)?;
    }
    run(out, "git", &["add", "-A"], dry)?;
    let _ = Command::new("git")
        .args(["-c", "commit.gpgsign=false", "commit", "-m", &format!("feat: scaffold {domain} landing page\n\nTier 2 of org-pages tree.")])
        .current_dir(out)
        .status();

    let exists = Command::new("gh").args(["repo", "view", gh_repo]).output()?.status.success();
    if !exists {
        run(
            out,
            "gh",
            &["repo", "create", gh_repo, "--public", "--description", &format!("Landing page for {domain}"), "--homepage", &format!("https://{domain}")],
            dry,
        )?;
    }
    let _ = Command::new("git").args(["remote", "add", "origin", &format!("https://github.com/{gh_repo}.git")]).current_dir(out).status();
    run(out, "git", &["push", "-u", "origin", "main"], dry)?;
    Ok(())
}

fn vercel_link_deploy_attach(out: &Path, slug: &str, domain: &str, dry: &bool) -> Result<()> {
    let project = format!("{slug}-landing");
    run(out, "vercel", &["link", "--yes", "--project", &project], dry)?;
    run(out, "vercel", &["deploy", "--prod", "--yes"], dry)?;
    run(out, "vercel", &["domains", "add", domain], dry)?;
    eprintln!("  ✓ vercel: {project} live at https://{domain}");
    Ok(())
}

// Tiny inline shellexpand replacement to avoid the dep — only `~/` expansion.
mod shellexpand {
    use std::borrow::Cow;
    pub fn tilde(s: &str) -> Cow<'_, str> {
        if let Some(rest) = s.strip_prefix("~/") {
            if let Ok(home) = std::env::var("HOME") {
                return Cow::Owned(format!("{home}/{rest}"));
            }
        }
        Cow::Borrowed(s)
    }
}
