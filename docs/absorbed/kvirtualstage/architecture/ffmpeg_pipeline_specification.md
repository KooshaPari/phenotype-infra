# FFmpeg Pipeline Specification
## Hardware-Accelerated Video Recording for KVirtualStage

**Architect:** Media_Recording_Architect  
**Date:** 2025-07-12  
**Version:** 1.0  

---

## 🎯 Pipeline Overview

This specification defines the FFmpeg-based video recording pipeline for KVirtualStage, designed to achieve professional 60fps 1080p recording with <5% frame drops using hardware acceleration.

## 🏗️ Pipeline Architecture

### Core Pipeline Components

```rust
pub struct FFmpegPipeline {
    // Input management
    input_source: InputSource,
    capture_settings: CaptureSettings,
    
    // Processing chain
    encoder: VideoEncoder,
    filters: FilterChain,
    quality_controller: QualityController,
    
    // Output management
    output_muxer: OutputMuxer,
    streaming_output: Option<StreamingOutput>,
    
    // Performance optimization
    hardware_acceleration: HardwareAcceleration,
    buffer_management: BufferManager,
}

pub struct CaptureSettings {
    display_source: String,          // ":1.0" for virtual display
    resolution: Resolution,          // 1920x1080, 1440x900, etc.
    framerate: f64,                 // 60.0, 30.0, etc.
    pixel_format: PixelFormat,      // bgr0, yuv420p, etc.
    color_space: ColorSpace,        // rec709, srgb, etc.
}
```

## 🎬 Input Source Configuration

### X11 Display Capture

**Primary Method**: Direct X11 display capture from virtual desktop containers.

```bash
# FFmpeg X11 capture command template
ffmpeg -f x11grab \
       -framerate 60 \
       -video_size 1920x1080 \
       -i :1.0+0,0 \
       -show_region 1 \
       -draw_mouse 1 \
       -follow_mouse centered \
       [encoding_options] \
       output.mp4
```

#### Advanced Input Options:

```rust
pub struct X11CaptureConfig {
    display: String,                    // ":1.0"
    offset: Point,                      // (0, 0) for full screen
    follow_mouse: MouseFollowMode,      // centered, 0, disabled
    show_region: bool,                  // Visual capture area indicator
    draw_mouse: bool,                   // Include cursor in recording
    grab_area: Option<Rectangle>,       // Specific region capture
}

pub enum MouseFollowMode {
    Disabled,           // Static capture area
    Centered,           // Keep mouse in center
    FollowOffset(i32),  // Follow with offset
}
```

### Container Display Integration

```rust
impl FFmpegPipeline {
    pub async fn configure_container_capture(
        &mut self,
        container_id: &str,
        display_number: u32
    ) -> Result<InputConfiguration> {
        // 1. Verify container display is active
        let display_info = self.verify_container_display(container_id, display_number).await?;
        
        // 2. Configure X11 connection
        let x11_config = X11CaptureConfig {
            display: format!(":{}", display_number),
            offset: Point::new(0, 0),
            follow_mouse: MouseFollowMode::Centered,
            show_region: false,
            draw_mouse: true,
            grab_area: None,
        };
        
        // 3. Test capture capability
        let test_result = self.test_capture_capability(&x11_config).await?;
        
        if !test_result.success {
            return Err(format!("Capture test failed: {}", test_result.error).into());
        }
        
        Ok(InputConfiguration {
            source_type: InputSourceType::X11,
            config: x11_config,
            capabilities: test_result.capabilities,
        })
    }
}
```

## ⚡ Hardware Acceleration Configuration

### Multi-Platform Acceleration Support

#### NVIDIA NVENC Configuration

```rust
pub struct NVENCConfig {
    device_id: Option<u32>,             // GPU device ID
    preset: NVENCPreset,                // slow, medium, fast
    rate_control: RateControlMode,      // CBR, VBR, CQP
    quality_level: u32,                 // 0-51 (CRF equivalent)
    b_frames: u32,                      // B-frame count
    spatial_aq: bool,                   // Spatial adaptive quantization
    temporal_aq: bool,                  // Temporal adaptive quantization
}

pub enum NVENCPreset {
    Slow,           // Best quality, slower encoding
    Medium,         // Balanced quality/speed
    Fast,           // Faster encoding, lower quality
    HighPerformance, // Maximum speed
    HighQuality,     // Maximum quality
}
```

**NVENC Pipeline Example:**

```bash
ffmpeg -f x11grab -framerate 60 -video_size 1920x1080 -i :1.0 \
       -c:v h264_nvenc \
       -preset slow \
       -rc vbr \
       -cq 18 \
       -spatial_aq 1 \
       -temporal_aq 1 \
       -b:v 10M \
       -maxrate 15M \
       -bufsize 30M \
       -profile:v high \
       -level 4.1 \
       -pix_fmt yuv420p \
       -movflags +faststart \
       output.mp4
```

#### Intel QuickSync Configuration

```rust
pub struct QuickSyncConfig {
    device_path: String,                // /dev/dri/renderD128
    preset: QuickSyncPreset,            // veryfast, fast, medium, slow
    global_quality: u32,                // 15-35 (quality level)
    look_ahead: bool,                   // Enable look-ahead
    look_ahead_depth: u32,              // 10-40 frames
    adaptive_i: bool,                   // Adaptive I-frame placement
    adaptive_b: bool,                   // Adaptive B-frame placement
}
```

**QuickSync Pipeline Example:**

```bash
ffmpeg -f x11grab -framerate 60 -video_size 1920x1080 -i :1.0 \
       -c:v h264_qsv \
       -preset medium \
       -global_quality 18 \
       -look_ahead 1 \
       -look_ahead_depth 40 \
       -adaptive_i 1 \
       -adaptive_b 1 \
       -b:v 8M \
       -maxrate 12M \
       -bufsize 24M \
       -profile:v high \
       -level 4.1 \
       -movflags +faststart \
       output.mp4
```

#### AMD AMF Configuration

```rust
pub struct AMFConfig {
    device_id: Option<u32>,             // GPU device ID
    usage: AMFUsage,                    // transcoding, webcam, lowlatency
    rate_control: AMFRateControl,       // CQP, CBR, VBR
    quality_preset: AMFQuality,         // speed, balanced, quality
    preanalysis: bool,                  // Pre-analysis pass
    vbaq: bool,                         // Variance based adaptive quantization
}
```

**AMF Pipeline Example:**

```bash
ffmpeg -f x11grab -framerate 60 -video_size 1920x1080 -i :1.0 \
       -c:v h264_amf \
       -usage transcoding \
       -rc cqp \
       -qp_i 18 \
       -qp_p 20 \
       -qp_b 22 \
       -quality quality \
       -preanalysis 1 \
       -vbaq 1 \
       -profile:v high \
       -level 4.1 \
       -movflags +faststart \
       output.mp4
```

### Hardware Detection Implementation

```rust
impl HardwareAcceleration {
    pub async fn detect_available_acceleration() -> Result<Vec<AccelerationType>> {
        let mut available = Vec::new();
        
        // Test NVIDIA NVENC
        if Self::test_nvenc_support().await? {
            available.push(AccelerationType::NVENC);
        }
        
        // Test Intel QuickSync
        if Self::test_quicksync_support().await? {
            available.push(AccelerationType::QuickSync);
        }
        
        // Test AMD AMF
        if Self::test_amf_support().await? {
            available.push(AccelerationType::AMF);
        }
        
        // Test VA-API (Linux)
        if Self::test_vaapi_support().await? {
            available.push(AccelerationType::VAAPI);
        }
        
        // Software encoding always available
        available.push(AccelerationType::Software);
        
        Ok(available)
    }
    
    async fn test_nvenc_support() -> Result<bool> {
        let test_command = Command::new("ffmpeg")
            .args(&[
                "-f", "lavfi",
                "-i", "testsrc2=duration=1:size=320x240:rate=30",
                "-c:v", "h264_nvenc",
                "-preset", "fast",
                "-f", "null",
                "-"
            ])
            .output()
            .await?;
        
        Ok(test_command.status.success())
    }
    
    async fn get_optimal_encoder_settings(
        &self,
        acceleration: &AccelerationType,
        target_quality: QualityLevel
    ) -> Result<EncoderSettings> {
        match (acceleration, target_quality) {
            (AccelerationType::NVENC, QualityLevel::High) => {
                Ok(EncoderSettings {
                    codec: "h264_nvenc".to_string(),
                    preset: "slow".to_string(),
                    quality_params: vec![
                        ("-rc", "vbr"),
                        ("-cq", "18"),
                        ("-spatial_aq", "1"),
                        ("-temporal_aq", "1"),
                        ("-b:v", "10M"),
                        ("-maxrate", "15M"),
                        ("-bufsize", "30M"),
                    ],
                })
            },
            
            (AccelerationType::QuickSync, QualityLevel::High) => {
                Ok(EncoderSettings {
                    codec: "h264_qsv".to_string(),
                    preset: "medium".to_string(),
                    quality_params: vec![
                        ("-global_quality", "18"),
                        ("-look_ahead", "1"),
                        ("-look_ahead_depth", "40"),
                        ("-b:v", "8M"),
                        ("-maxrate", "12M"),
                        ("-bufsize", "24M"),
                    ],
                })
            },
            
            (AccelerationType::Software, QualityLevel::High) => {
                Ok(EncoderSettings {
                    codec: "libx264".to_string(),
                    preset: "fast".to_string(),
                    quality_params: vec![
                        ("-crf", "20"),
                        ("-tune", "zerolatency"),
                        ("-threads", "0"),
                        ("-b:v", "6M"),
                        ("-maxrate", "9M"),
                        ("-bufsize", "18M"),
                    ],
                })
            },
            
            _ => Err("Unsupported acceleration/quality combination".into()),
        }
    }
}
```

## 🎚️ Quality Optimization Profiles

### Recording Quality Presets

```rust
pub enum QualityLevel {
    Maximum,        // Best quality, highest resource usage
    High,           // Professional quality, balanced resources
    Medium,         // Good quality, moderate resources
    Low,            // Acceptable quality, minimal resources
    Adaptive,       // Dynamic quality based on performance
}

pub struct QualityProfile {
    level: QualityLevel,
    target_bitrate: BitrateConfig,
    encoding_settings: EncodingSettings,
    performance_limits: PerformanceLimits,
}
```

#### Maximum Quality Profile

```rust
pub const MAXIMUM_QUALITY_PROFILE: QualityProfile = QualityProfile {
    level: QualityLevel::Maximum,
    target_bitrate: BitrateConfig {
        video_bitrate: 15_000_000,      // 15 Mbps
        max_bitrate: 20_000_000,        // 20 Mbps
        buffer_size: 40_000_000,        // 40 Mbps buffer
    },
    encoding_settings: EncodingSettings {
        crf_value: 16,                  // Very high quality
        preset: "slower",               // Best compression
        tune: None,                     // No special tuning
        profile: "high",                // H.264 High Profile
        level: "4.2",                   // Support for high bitrates
    },
    performance_limits: PerformanceLimits {
        max_cpu_usage: 0.9,             // 90% CPU allowed
        max_encoding_latency: Duration::from_millis(200),
        min_framerate: 55.0,            // Minimum acceptable FPS
    },
};
```

#### Adaptive Quality Implementation

```rust
impl QualityController {
    pub async fn adjust_quality_dynamically(
        &mut self,
        performance_metrics: &PerformanceMetrics
    ) -> Result<QualityAdjustment> {
        let mut adjustment = QualityAdjustment::new();
        
        // Check frame drop rate
        if performance_metrics.frame_drop_rate > 0.05 {
            // More than 5% frame drops - reduce quality
            adjustment.crf_delta = 2;
            adjustment.preset_change = Some("faster".to_string());
            adjustment.bitrate_reduction = 0.8;
        }
        
        // Check encoding latency
        if performance_metrics.encoding_latency > Duration::from_millis(100) {
            // High encoding latency - reduce complexity
            adjustment.preset_change = Some("veryfast".to_string());
            adjustment.tune_change = Some("zerolatency".to_string());
        }
        
        // Check CPU usage
        if performance_metrics.cpu_usage > 0.85 {
            // High CPU usage - optimize for speed
            adjustment.thread_optimization = true;
            adjustment.encode_speed_boost = true;
        }
        
        // Apply adjustments
        self.apply_quality_adjustment(&adjustment).await?;
        
        Ok(adjustment)
    }
}
```

## 🔧 Advanced Filter Chain

### Video Processing Filters

```rust
pub struct FilterChain {
    input_filters: Vec<InputFilter>,
    processing_filters: Vec<ProcessingFilter>,
    output_filters: Vec<OutputFilter>,
}

pub enum ProcessingFilter {
    ScaleFilter {
        width: u32,
        height: u32,
        algorithm: ScaleAlgorithm,
    },
    FramerateFilter {
        input_fps: f64,
        output_fps: f64,
        interpolation: InterpolationMode,
    },
    ColorspaceFilter {
        input_colorspace: ColorSpace,
        output_colorspace: ColorSpace,
    },
    NoiseReductionFilter {
        strength: f64,
        temporal: bool,
    },
    SharpeningFilter {
        strength: f64,
        radius: f64,
    },
}
```

#### Cursor Enhancement Filter

```rust
pub struct CursorEnhancementFilter {
    cursor_size_multiplier: f64,       // 1.0-2.0 for cursor emphasis
    cursor_highlight: bool,            // Add highlight around cursor
    cursor_trail: bool,                // Add motion trail
    cursor_smoothing: bool,            // Smooth cursor movement
}

impl CursorEnhancementFilter {
    pub fn build_filter_string(&self) -> String {
        let mut filters = Vec::new();
        
        if self.cursor_highlight {
            filters.push("drawbox=x=cursor_x-10:y=cursor_y-10:w=20:h=20:color=yellow@0.3:t=2".to_string());
        }
        
        if self.cursor_smoothing {
            filters.push("minterpolate=fps=60:mi_mode=mci".to_string());
        }
        
        filters.join(",")
    }
}
```

#### Quality Enhancement Filters

```bash
# High-quality upscaling filter
-vf "scale=1920:1080:flags=lanczos:force_original_aspect_ratio=decrease"

# Noise reduction for clean output
-vf "nlmeans=s=1.0:p=7:r=15"

# Sharpening for crisp details
-vf "unsharp=luma_msize_x=5:luma_msize_y=5:luma_amount=1.0"

# Color correction and enhancement
-vf "eq=contrast=1.1:brightness=0.02:saturation=1.1"

# Combined filter chain
-vf "scale=1920:1080:flags=lanczos,nlmeans=s=0.8:p=5:r=11,unsharp=luma_msize_x=3:luma_msize_y=3:luma_amount=0.8,eq=contrast=1.05:saturation=1.05"
```

## 📊 Real-Time Monitoring

### Performance Metrics Collection

```rust
pub struct PipelineMonitor {
    frame_metrics: FrameMetrics,
    encoding_metrics: EncodingMetrics,
    resource_metrics: ResourceMetrics,
    quality_metrics: QualityMetrics,
}

pub struct FrameMetrics {
    frames_captured: u64,
    frames_dropped: u64,
    frames_duplicated: u64,
    average_frame_time: Duration,
    frame_rate_variance: f64,
}

impl PipelineMonitor {
    pub async fn collect_real_time_metrics(&mut self) -> Result<PipelineMetrics> {
        // Collect FFmpeg statistics
        let ffmpeg_stats = self.parse_ffmpeg_output().await?;
        
        // Monitor system resources
        let system_metrics = self.collect_system_metrics().await?;
        
        // Calculate derived metrics
        let quality_score = self.calculate_quality_score(&ffmpeg_stats)?;
        let performance_score = self.calculate_performance_score(&system_metrics)?;
        
        Ok(PipelineMetrics {
            timestamp: Instant::now(),
            frame_metrics: ffmpeg_stats.frame_metrics,
            encoding_metrics: ffmpeg_stats.encoding_metrics,
            resource_metrics: system_metrics,
            quality_score,
            performance_score,
        })
    }
    
    async fn parse_ffmpeg_output(&self) -> Result<FFmpegStatistics> {
        // Parse FFmpeg progress output
        // Example: frame= 1234 fps=59.8 q=18.0 size= 45678kB time=00:00:20.56 bitrate=18234.5kbits/s speed=0.996x
        
        let stats_regex = Regex::new(
            r"frame=\s*(\d+)\s+fps=\s*([\d.]+)\s+q=\s*([\d.]+)\s+size=\s*(\d+)kB\s+time=(\S+)\s+bitrate=\s*([\d.]+)kbits/s\s+speed=\s*([\d.]+)x"
        )?;
        
        // Implementation details for parsing...
        
        Ok(FFmpegStatistics {
            frame_count: 1234,
            current_fps: 59.8,
            current_quality: 18.0,
            output_size_kb: 45678,
            encoding_speed: 0.996,
            bitrate_kbps: 18234.5,
        })
    }
}
```

## 🚀 Production Pipeline Commands

### Complete Pipeline Examples

#### Professional Recording Pipeline (NVENC)

```bash
#!/bin/bash
# Professional quality recording with NVIDIA hardware acceleration

ffmpeg \
    -f x11grab \
    -framerate 60 \
    -video_size 1920x1080 \
    -show_region 0 \
    -draw_mouse 1 \
    -follow_mouse centered \
    -i :1.0+0,0 \
    \
    -c:v h264_nvenc \
    -preset slow \
    -rc vbr \
    -cq 18 \
    -spatial_aq 1 \
    -temporal_aq 1 \
    -b:v 10M \
    -maxrate 15M \
    -bufsize 30M \
    -profile:v high \
    -level 4.1 \
    -pix_fmt yuv420p \
    \
    -vf "scale=1920:1080:flags=lanczos:force_original_aspect_ratio=decrease,unsharp=luma_msize_x=3:luma_msize_y=3:luma_amount=0.5" \
    \
    -movflags +faststart \
    -fflags +genpts \
    -avoid_negative_ts make_zero \
    \
    -t 300 \
    -y output_professional.mp4
```

#### Adaptive Quality Pipeline

```bash
#!/bin/bash
# Adaptive quality pipeline that adjusts based on performance

INITIAL_CRF=20
MAX_CPU_USAGE=80
TARGET_FPS=60

# Monitor CPU usage and adjust quality
while true; do
    CPU_USAGE=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | awk -F'%' '{print $1}')
    
    if (( $(echo "$CPU_USAGE > $MAX_CPU_USAGE" | bc -l) )); then
        # Reduce quality if CPU usage too high
        CURRENT_CRF=$((CURRENT_CRF + 2))
        PRESET="faster"
    elif (( $(echo "$CPU_USAGE < 50" | bc -l) )); then
        # Increase quality if CPU usage low
        CURRENT_CRF=$((CURRENT_CRF - 1))
        PRESET="medium"
    fi
    
    # Apply new settings (restart recording with new parameters)
    # Implementation would restart FFmpeg process with new settings
    
    sleep 10
done
```

## 🏆 Pipeline Performance Targets

### Quality Benchmarks

| Metric | Target | Minimum Acceptable |
|--------|--------|--------------------|
| **Frame Rate** | 60 FPS | 55 FPS |
| **Frame Drops** | <2% | <5% |
| **Encoding Latency** | <50ms | <100ms |
| **Visual Quality** | CRF 18-20 | CRF 22 |
| **File Size Efficiency** | 8-12 MB/min | 15 MB/min |
| **CPU Usage** | <70% | <85% |
| **Memory Usage** | <2GB | <4GB |

### Hardware Acceleration Performance

| Acceleration Type | Encoding Speed | Quality | CPU Usage |
|-------------------|----------------|---------|-----------|
| **NVIDIA NVENC** | 0.8-1.2x realtime | Excellent | 10-20% |
| **Intel QuickSync** | 0.9-1.1x realtime | Very Good | 15-25% |
| **AMD AMF** | 0.7-1.0x realtime | Good | 20-30% |
| **Software x264** | 0.3-0.8x realtime | Excellent | 80-95% |

This FFmpeg pipeline specification provides the foundation for professional-quality video recording in KVirtualStage with optimal hardware acceleration and adaptive quality control.