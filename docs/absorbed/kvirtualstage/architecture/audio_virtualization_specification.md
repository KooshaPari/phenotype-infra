# Audio Virtualization Specification
## PipeWire-Based Virtual Audio System for KVirtualStage

**Architect:** Media_Recording_Architect  
**Date:** 2025-07-12  
**Version:** 1.0  

---

## 🎯 System Overview

This specification defines a comprehensive audio virtualization framework for KVirtualStage, enabling virtual audio devices, TTS/STT integration, and seamless audio capture within containerized desktop environments.

## 🏗️ Architecture Components

### Core Audio Infrastructure

```rust
pub struct AudioVirtualizationSystem {
    // Core components
    pipewire_manager: PipeWireManager,
    virtual_device_manager: VirtualDeviceManager,
    audio_router: AudioRouter,
    session_manager: AudioSessionManager,
    
    // Processing engines
    tts_engine: TextToSpeechEngine,
    stt_engine: SpeechToTextEngine,
    audio_processor: AudioProcessor,
    effects_engine: AudioEffectsEngine,
    
    // Integration interfaces
    container_bridge: ContainerAudioBridge,
    recording_integration: RecordingIntegration,
    automation_hooks: AutomationAudioHooks,
}

pub struct VirtualDeviceManager {
    virtual_sources: HashMap<DeviceId, VirtualAudioSource>,
    virtual_sinks: HashMap<DeviceId, VirtualAudioSink>,
    device_registry: DeviceRegistry,
    routing_matrix: AudioRoutingMatrix,
}

pub struct PipeWireManager {
    core_context: PipeWireContext,
    device_factory: DeviceFactory,
    stream_manager: StreamManager,
    node_manager: NodeManager,
}
```

## 🎤 Virtual Device Implementation

### Virtual Microphone System

**Core Functionality**: Create virtual microphone devices that can inject TTS audio and capture voice commands.

```rust
pub struct VirtualMicrophone {
    device_id: DeviceId,
    device_name: String,
    pipewire_node_id: u32,
    
    // Audio properties
    sample_rate: u32,           // 48000 Hz standard
    channels: u32,              // 2 (stereo) or 1 (mono)
    format: AudioFormat,        // F32LE, S16LE, etc.
    buffer_size: u32,           // 1024 samples default
    
    // State management
    state: DeviceState,
    clients: Vec<AudioClient>,
    injection_queue: AudioQueue,
    
    // Performance monitoring
    metrics: AudioMetrics,
}

pub enum AudioFormat {
    F32LE,      // 32-bit float little endian
    S32LE,      // 32-bit signed integer little endian
    S16LE,      // 16-bit signed integer little endian
    U8,         // 8-bit unsigned integer
}

pub enum DeviceState {
    Inactive,
    Active,
    Suspended,
    Error(String),
}
```

#### Virtual Microphone Creation

```rust
impl VirtualMicrophone {
    pub async fn create_virtual_microphone(
        session_id: &str,
        config: VirtualMicConfig
    ) -> Result<VirtualMicrophone> {
        let device_name = format!("kvs_virtual_mic_{}", session_id);
        
        // Create PipeWire node using pw-cli
        let node_creation_command = format!(
            r#"pw-cli create-node adapter {{
                factory.name = support.null-audio-sink
                node.name = "{}"
                node.description = "KVirtualStage Virtual Microphone"
                media.class = Audio/Source
                audio.format = {}
                audio.rate = {}
                audio.channels = {}
                audio.position = [FL FR]
                node.pause-on-idle = false
                object.linger = true
            }}"#,
            device_name,
            config.format.to_pipewire_string(),
            config.sample_rate,
            config.channels
        );
        
        let output = Command::new("pw-cli")
            .arg("create-node")
            .arg(&node_creation_command)
            .output()
            .await?;
        
        if !output.status.success() {
            return Err(format!(
                "Failed to create virtual microphone: {}",
                String::from_utf8_lossy(&output.stderr)
            ).into());
        }
        
        // Parse node ID from output
        let node_id = Self::parse_node_id_from_output(&output.stdout)?;
        
        // Wait for node to become active
        Self::wait_for_node_activation(node_id).await?;
        
        Ok(VirtualMicrophone {
            device_id: DeviceId::new(),
            device_name,
            pipewire_node_id: node_id,
            sample_rate: config.sample_rate,
            channels: config.channels,
            format: config.format,
            buffer_size: config.buffer_size,
            state: DeviceState::Active,
            clients: Vec::new(),
            injection_queue: AudioQueue::new(config.buffer_size),
            metrics: AudioMetrics::new(),
        })
    }
    
    pub async fn inject_audio_data(
        &mut self,
        audio_data: &AudioBuffer
    ) -> Result<InjectionResult> {
        // Validate audio format compatibility
        self.validate_audio_format(audio_data)?;
        
        // Convert to device format if necessary
        let converted_data = if audio_data.format != self.format {
            self.convert_audio_format(audio_data).await?
        } else {
            audio_data.clone()
        };
        
        // Queue audio data for injection
        let injection_id = self.injection_queue.enqueue(converted_data).await?;
        
        // Trigger PipeWire node to read from queue
        self.trigger_audio_injection(injection_id).await?;
        
        // Update metrics
        self.metrics.update_injection_stats(&audio_data);
        
        Ok(InjectionResult {
            injection_id,
            timestamp: Instant::now(),
            duration: audio_data.duration(),
            success: true,
        })
    }
    
    async fn trigger_audio_injection(&self, injection_id: InjectionId) -> Result<()> {
        // Use PipeWire Stream API to inject audio
        let injection_command = format!(
            "pw-cat --playback --format={} --rate={} --channels={} --target={} /tmp/audio_injection_{}",
            self.format.to_string(),
            self.sample_rate,
            self.channels,
            self.pipewire_node_id,
            injection_id
        );
        
        let result = Command::new("sh")
            .arg("-c")
            .arg(&injection_command)
            .spawn()?;
        
        Ok(())
    }
}
```

### Virtual Speaker System

```rust
pub struct VirtualSpeaker {
    device_id: DeviceId,
    device_name: String,
    pipewire_node_id: u32,
    
    // Audio capture
    capture_stream: Option<CaptureStream>,
    audio_buffer: CircularBuffer<AudioSample>,
    
    // Processing
    audio_processor: AudioProcessor,
    recording_sync: RecordingSync,
    
    // Output routing
    output_targets: Vec<OutputTarget>,
}

impl VirtualSpeaker {
    pub async fn create_virtual_speaker(
        session_id: &str,
        config: VirtualSpeakerConfig
    ) -> Result<VirtualSpeaker> {
        let device_name = format!("kvs_virtual_speaker_{}", session_id);
        
        // Create PipeWire sink node
        let sink_creation_command = format!(
            r#"pw-cli create-node adapter {{
                factory.name = support.null-audio-sink
                node.name = "{}"
                node.description = "KVirtualStage Virtual Speaker"
                media.class = Audio/Sink
                audio.format = F32LE
                audio.rate = 48000
                audio.channels = 2
                node.pause-on-idle = false
                object.linger = true
            }}"#,
            device_name
        );
        
        let output = Command::new("pw-cli")
            .args(&["create-node", &sink_creation_command])
            .output()
            .await?;
        
        let node_id = Self::parse_node_id_from_output(&output.stdout)?;
        
        // Setup capture stream to monitor audio
        let capture_stream = Self::setup_capture_stream(node_id).await?;
        
        Ok(VirtualSpeaker {
            device_id: DeviceId::new(),
            device_name,
            pipewire_node_id: node_id,
            capture_stream: Some(capture_stream),
            audio_buffer: CircularBuffer::new(48000 * 2), // 1 second buffer
            audio_processor: AudioProcessor::new(),
            recording_sync: RecordingSync::new(),
            output_targets: Vec::new(),
        })
    }
    
    pub async fn capture_audio_output(&mut self) -> Result<AudioCapture> {
        if let Some(ref mut stream) = self.capture_stream {
            let audio_data = stream.read_available_samples().await?;
            
            // Process captured audio
            let processed_audio = self.audio_processor.process(&audio_data).await?;
            
            // Store in buffer for recording integration
            self.audio_buffer.write(&processed_audio);
            
            // Sync with recording system
            self.recording_sync.notify_audio_available(&processed_audio).await?;
            
            Ok(AudioCapture {
                timestamp: Instant::now(),
                audio_data: processed_audio,
                sample_rate: 48000,
                channels: 2,
            })
        } else {
            Err("Capture stream not initialized".into())
        }
    }
}
```

## 🗣️ Text-to-Speech Integration

### Multi-Provider TTS Engine

```rust
pub struct TextToSpeechEngine {
    providers: HashMap<String, Box<dyn TTSProvider>>,
    default_provider: String,
    voice_profiles: HashMap<String, VoiceProfile>,
    audio_cache: TTSCache,
    quality_settings: TTSQualitySettings,
}

pub trait TTSProvider: Send + Sync {
    async fn synthesize_speech(
        &self,
        text: &str,
        voice_config: &VoiceConfig
    ) -> Result<AudioBuffer>;
    
    async fn get_available_voices(&self) -> Result<Vec<VoiceInfo>>;
    
    fn get_provider_name(&self) -> &str;
    
    fn get_supported_formats(&self) -> Vec<AudioFormat>;
}

pub struct VoiceConfig {
    voice_id: String,
    speed: f64,              // 0.25-4.0, 1.0 = normal
    pitch: f64,              // 0.5-2.0, 1.0 = normal
    volume: f64,             // 0.0-1.0
    emotion: Option<String>, // happy, sad, excited, etc.
    style: Option<String>,   // casual, professional, dramatic
}
```

#### ElevenLabs TTS Provider

```rust
pub struct ElevenLabsProvider {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
    rate_limiter: RateLimiter,
}

impl TTSProvider for ElevenLabsProvider {
    async fn synthesize_speech(
        &self,
        text: &str,
        voice_config: &VoiceConfig
    ) -> Result<AudioBuffer> {
        // Rate limiting
        self.rate_limiter.wait_for_permission().await?;
        
        // Prepare request
        let request_body = json!({
            "text": text,
            "model_id": "eleven_monolingual_v1",
            "voice_settings": {
                "stability": 0.5,
                "similarity_boost": 0.75,
                "style": voice_config.style.unwrap_or_default(),
                "use_speaker_boost": true
            }
        });
        
        let url = format!("{}/v1/text-to-speech/{}", self.base_url, voice_config.voice_id);
        
        let response = self.client
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(format!("ElevenLabs API error: {}", response.status()).into());
        }
        
        let audio_data = response.bytes().await?;
        
        // Convert MP3 to raw audio format
        let audio_buffer = self.convert_mp3_to_raw(&audio_data).await?;
        
        Ok(audio_buffer)
    }
    
    async fn convert_mp3_to_raw(&self, mp3_data: &[u8]) -> Result<AudioBuffer> {
        // Use FFmpeg to convert MP3 to raw PCM
        let temp_input = "/tmp/tts_input.mp3";
        let temp_output = "/tmp/tts_output.wav";
        
        // Write MP3 data to temporary file
        tokio::fs::write(temp_input, mp3_data).await?;
        
        // Convert using FFmpeg
        let output = Command::new("ffmpeg")
            .args(&[
                "-i", temp_input,
                "-f", "wav",
                "-ar", "48000",
                "-ac", "2",
                "-sample_fmt", "f32le",
                "-y", temp_output
            ])
            .output()
            .await?;
        
        if !output.status.success() {
            return Err("FFmpeg conversion failed".into());
        }
        
        // Read converted audio
        let wav_data = tokio::fs::read(temp_output).await?;
        let audio_buffer = AudioBuffer::from_wav_data(&wav_data)?;
        
        // Cleanup
        tokio::fs::remove_file(temp_input).await.ok();
        tokio::fs::remove_file(temp_output).await.ok();
        
        Ok(audio_buffer)
    }
}
```

#### OpenAI TTS Provider

```rust
pub struct OpenAITTSProvider {
    api_key: String,
    client: reqwest::Client,
    model: String,  // tts-1, tts-1-hd
}

impl TTSProvider for OpenAITTSProvider {
    async fn synthesize_speech(
        &self,
        text: &str,
        voice_config: &VoiceConfig
    ) -> Result<AudioBuffer> {
        let request_body = json!({
            "model": self.model,
            "input": text,
            "voice": voice_config.voice_id,
            "response_format": "wav",
            "speed": voice_config.speed
        });
        
        let response = self.client
            .post("https://api.openai.com/v1/audio/speech")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(format!("OpenAI API error: {}", response.status()).into());
        }
        
        let audio_data = response.bytes().await?;
        let audio_buffer = AudioBuffer::from_wav_data(&audio_data)?;
        
        Ok(audio_buffer)
    }
}
```

### TTS Integration with Virtual Devices

```rust
impl TextToSpeechEngine {
    pub async fn speak_text_to_virtual_device(
        &mut self,
        text: &str,
        virtual_mic: &mut VirtualMicrophone,
        voice_config: &VoiceConfig
    ) -> Result<SpeechInjectionResult> {
        // 1. Generate speech audio
        let audio_buffer = self.synthesize_speech(text, voice_config).await?;
        
        // 2. Apply audio processing if needed
        let processed_audio = self.apply_audio_processing(&audio_buffer, voice_config).await?;
        
        // 3. Inject into virtual microphone
        let injection_result = virtual_mic.inject_audio_data(&processed_audio).await?;
        
        // 4. Monitor injection progress
        let monitoring_task = self.monitor_speech_injection(&injection_result).await?;
        
        Ok(SpeechInjectionResult {
            injection_id: injection_result.injection_id,
            text: text.to_string(),
            duration: processed_audio.duration(),
            voice_config: voice_config.clone(),
            monitoring_task,
        })
    }
    
    async fn apply_audio_processing(
        &self,
        audio_buffer: &AudioBuffer,
        voice_config: &VoiceConfig
    ) -> Result<AudioBuffer> {
        let mut processed = audio_buffer.clone();
        
        // Apply volume adjustment
        if voice_config.volume != 1.0 {
            processed.apply_volume(voice_config.volume)?;
        }
        
        // Apply pitch adjustment
        if voice_config.pitch != 1.0 {
            processed.apply_pitch_shift(voice_config.pitch).await?;
        }
        
        // Apply speed adjustment (if not handled by TTS provider)
        if voice_config.speed != 1.0 && !self.provider_handles_speed() {
            processed.apply_speed_change(voice_config.speed).await?;
        }
        
        // Apply audio effects for realism
        processed.apply_natural_effects().await?;
        
        Ok(processed)
    }
}
```

## 🎧 Speech-to-Text Integration

### Real-Time STT Engine

```rust
pub struct SpeechToTextEngine {
    providers: HashMap<String, Box<dyn STTProvider>>,
    default_provider: String,
    recognition_config: RecognitionConfig,
    language_models: HashMap<String, LanguageModel>,
    
    // Real-time processing
    audio_stream: Option<AudioStream>,
    processing_buffer: CircularBuffer<AudioSample>,
    recognition_pipeline: RecognitionPipeline,
}

pub trait STTProvider: Send + Sync {
    async fn transcribe_audio(
        &self,
        audio_buffer: &AudioBuffer,
        config: &RecognitionConfig
    ) -> Result<TranscriptionResult>;
    
    async fn start_real_time_recognition(
        &self,
        config: &RecognitionConfig
    ) -> Result<Box<dyn RealTimeRecognition>>;
}

pub struct RecognitionConfig {
    language: String,               // "en-US", "es-ES", etc.
    sample_rate: u32,               // Audio sample rate
    enable_automatic_punctuation: bool,
    enable_word_time_offsets: bool,
    enable_speaker_diarization: bool,
    vocabulary_filter: Option<Vec<String>>,
}
```

#### Whisper STT Provider

```rust
pub struct WhisperSTTProvider {
    model_path: PathBuf,
    model_size: WhisperModelSize,
    device: WhisperDevice,
}

pub enum WhisperModelSize {
    Tiny,       // 39 MB, ~32x realtime
    Base,       // 74 MB, ~16x realtime
    Small,      // 244 MB, ~6x realtime
    Medium,     // 769 MB, ~2x realtime
    Large,      // 1550 MB, ~1x realtime
}

impl STTProvider for WhisperSTTProvider {
    async fn transcribe_audio(
        &self,
        audio_buffer: &AudioBuffer,
        config: &RecognitionConfig
    ) -> Result<TranscriptionResult> {
        // Convert audio buffer to WAV file for Whisper
        let temp_audio_file = self.prepare_audio_for_whisper(audio_buffer).await?;
        
        // Run Whisper transcription
        let whisper_command = format!(
            "whisper {} --model {} --language {} --output_format json --output_dir /tmp",
            temp_audio_file.display(),
            self.model_size.to_string(),
            config.language
        );
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&whisper_command)
            .output()
            .await?;
        
        if !output.status.success() {
            return Err(format!("Whisper transcription failed: {}", 
                             String::from_utf8_lossy(&output.stderr)).into());
        }
        
        // Parse Whisper JSON output
        let transcription_data = self.parse_whisper_output(&temp_audio_file).await?;
        
        // Cleanup
        tokio::fs::remove_file(temp_audio_file).await.ok();
        
        Ok(TranscriptionResult {
            text: transcription_data.text,
            confidence: transcription_data.confidence,
            word_timestamps: transcription_data.word_timestamps,
            language: transcription_data.detected_language,
            duration: audio_buffer.duration(),
        })
    }
}
```

### Real-Time Voice Command Recognition

```rust
pub struct VoiceCommandEngine {
    stt_engine: SpeechToTextEngine,
    command_parser: CommandParser,
    action_executor: ActionExecutor,
    
    // Voice activation
    wake_word_detector: WakeWordDetector,
    voice_activity_detector: VoiceActivityDetector,
    
    // Command processing
    command_history: Vec<VoiceCommand>,
    context_manager: VoiceContext,
}

pub struct VoiceCommand {
    command_id: CommandId,
    raw_text: String,
    parsed_intent: Intent,
    confidence: f64,
    timestamp: Instant,
    execution_result: Option<ExecutionResult>,
}

impl VoiceCommandEngine {
    pub async fn start_voice_command_listening(
        &mut self,
        virtual_speaker: &VirtualSpeaker
    ) -> Result<VoiceCommandSession> {
        // 1. Setup audio capture from virtual speaker
        let audio_stream = virtual_speaker.create_capture_stream().await?;
        
        // 2. Initialize voice activity detection
        let vad_stream = self.voice_activity_detector.wrap_stream(audio_stream).await?;
        
        // 3. Setup wake word detection
        let wake_word_stream = self.wake_word_detector.wrap_stream(vad_stream).await?;
        
        // 4. Start command recognition loop
        let command_session = VoiceCommandSession::new();
        let session_id = command_session.session_id.clone();
        
        // Spawn command processing task
        let command_processor = self.clone();
        tokio::spawn(async move {
            command_processor.process_voice_commands(wake_word_stream, session_id).await
        });
        
        Ok(command_session)
    }
    
    async fn process_voice_commands(
        &self,
        mut audio_stream: impl AudioStream,
        session_id: SessionId
    ) -> Result<()> {
        while let Some(audio_chunk) = audio_stream.next().await {
            // Check for voice activity
            if self.voice_activity_detector.detect_speech(&audio_chunk) {
                // Accumulate audio until silence
                let complete_utterance = self.accumulate_utterance(&mut audio_stream).await?;
                
                // Transcribe speech
                let transcription = self.stt_engine.transcribe_audio(
                    &complete_utterance,
                    &self.get_recognition_config()
                ).await?;
                
                // Parse command intent
                if let Ok(intent) = self.command_parser.parse_intent(&transcription.text) {
                    // Execute command
                    let execution_result = self.action_executor.execute_intent(&intent).await?;
                    
                    // Log command
                    let voice_command = VoiceCommand {
                        command_id: CommandId::new(),
                        raw_text: transcription.text,
                        parsed_intent: intent,
                        confidence: transcription.confidence,
                        timestamp: Instant::now(),
                        execution_result: Some(execution_result),
                    };
                    
                    self.log_voice_command(session_id.clone(), voice_command).await?;
                }
            }
        }
        
        Ok(())
    }
}
```

## 🔌 Container Integration

### Audio Bridge for Containerized Desktops

```rust
pub struct ContainerAudioBridge {
    container_manager: ContainerManager,
    audio_routing: AudioRouting,
    pulse_audio_bridge: PulseAudioBridge,
    pipewire_bridge: PipeWireBridge,
}

impl ContainerAudioBridge {
    pub async fn setup_container_audio(
        &mut self,
        container_id: &str,
        session_config: &AudioSessionConfig
    ) -> Result<ContainerAudioSession> {
        // 1. Create virtual audio devices for container
        let virtual_mic = VirtualMicrophone::create_virtual_microphone(
            &format!("container_{}", container_id),
            session_config.microphone_config.clone()
        ).await?;
        
        let virtual_speaker = VirtualSpeaker::create_virtual_speaker(
            &format!("container_{}", container_id),
            session_config.speaker_config.clone()
        ).await?;
        
        // 2. Setup PulseAudio bridge for container
        self.setup_pulse_audio_bridge(container_id, &virtual_mic, &virtual_speaker).await?;
        
        // 3. Configure container audio environment
        self.configure_container_audio_environment(container_id).await?;
        
        // 4. Create audio routing
        let routing = self.create_audio_routing(container_id, &virtual_mic, &virtual_speaker).await?;
        
        Ok(ContainerAudioSession {
            container_id: container_id.to_string(),
            virtual_microphone: virtual_mic,
            virtual_speaker: virtual_speaker,
            audio_routing: routing,
            pulse_bridge: self.pulse_audio_bridge.clone(),
        })
    }
    
    async fn setup_pulse_audio_bridge(
        &self,
        container_id: &str,
        virtual_mic: &VirtualMicrophone,
        virtual_speaker: &VirtualSpeaker
    ) -> Result<()> {
        // Create PulseAudio configuration for container
        let pulse_config = format!(
            r#"
# Container audio configuration
load-module module-pipe-source source_name=kvs_mic_{} file=/tmp/kvs_audio_in_{}
load-module module-pipe-sink sink_name=kvs_speaker_{} file=/tmp/kvs_audio_out_{}

# Set default devices
set-default-source kvs_mic_{}
set-default-sink kvs_speaker_{}
            "#,
            container_id, container_id,
            container_id, container_id,
            container_id, container_id
        );
        
        // Mount audio configuration into container
        let config_path = format!("/tmp/pulse_config_{}.pa", container_id);
        tokio::fs::write(&config_path, pulse_config).await?;
        
        // Execute audio setup inside container
        let setup_command = format!(
            "docker exec {} sh -c 'pulseaudio --kill; pulseaudio --start --file={}'",
            container_id, config_path
        );
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&setup_command)
            .output()
            .await?;
        
        if !output.status.success() {
            return Err(format!("PulseAudio setup failed: {}", 
                             String::from_utf8_lossy(&output.stderr)).into());
        }
        
        Ok(())
    }
}
```

## 📊 Audio Quality Monitoring

### Real-Time Audio Metrics

```rust
pub struct AudioQualityMonitor {
    quality_analyzer: AudioQualityAnalyzer,
    latency_monitor: LatencyMonitor,
    distortion_detector: DistortionDetector,
    volume_monitor: VolumeMonitor,
}

pub struct AudioQualityMetrics {
    sample_rate: u32,
    bit_depth: u32,
    channels: u32,
    
    // Quality measurements
    signal_to_noise_ratio: f64,    // dB
    total_harmonic_distortion: f64, // %
    dynamic_range: f64,             // dB
    frequency_response: FrequencyResponse,
    
    // Performance metrics
    latency: Duration,
    jitter: Duration,
    buffer_underruns: u64,
    buffer_overruns: u64,
    
    // Volume metrics
    peak_level: f64,               // dBFS
    rms_level: f64,                // dBFS
    loudness: f64,                 // LUFS
}

impl AudioQualityMonitor {
    pub async fn analyze_audio_quality(
        &mut self,
        audio_buffer: &AudioBuffer
    ) -> Result<AudioQualityMetrics> {
        let start_time = Instant::now();
        
        // Analyze signal quality
        let snr = self.quality_analyzer.calculate_snr(audio_buffer).await?;
        let thd = self.distortion_detector.calculate_thd(audio_buffer).await?;
        let dynamic_range = self.quality_analyzer.calculate_dynamic_range(audio_buffer).await?;
        
        // Measure latency
        let latency = self.latency_monitor.measure_current_latency().await?;
        
        // Analyze volume characteristics
        let volume_metrics = self.volume_monitor.analyze_volume(audio_buffer).await?;
        
        // Frequency response analysis
        let freq_response = self.quality_analyzer.analyze_frequency_response(audio_buffer).await?;
        
        Ok(AudioQualityMetrics {
            sample_rate: audio_buffer.sample_rate,
            bit_depth: audio_buffer.bit_depth,
            channels: audio_buffer.channels,
            signal_to_noise_ratio: snr,
            total_harmonic_distortion: thd,
            dynamic_range,
            frequency_response: freq_response,
            latency,
            jitter: self.latency_monitor.get_jitter(),
            buffer_underruns: self.get_buffer_underrun_count(),
            buffer_overruns: self.get_buffer_overrun_count(),
            peak_level: volume_metrics.peak_level,
            rms_level: volume_metrics.rms_level,
            loudness: volume_metrics.loudness,
        })
    }
}
```

## 🏆 Performance Targets

### Audio System Benchmarks

| Metric | Target | Minimum Acceptable |
|--------|--------|--------------------|
| **Latency** | <20ms | <50ms |
| **Sample Rate** | 48kHz | 44.1kHz |
| **Bit Depth** | 24-bit | 16-bit |
| **SNR** | >80dB | >60dB |
| **THD** | <0.1% | <1% |
| **CPU Usage** | <10% | <20% |
| **Memory Usage** | <256MB | <512MB |

### TTS Quality Targets

| Provider | Latency | Quality | Cost per 1000 chars |
|----------|---------|---------|---------------------|
| **ElevenLabs** | 1-3s | Excellent | $0.30 |
| **OpenAI** | 0.5-2s | Very Good | $0.015 |
| **Local Whisper** | 0.1-1s | Good | Free |
| **Azure** | 0.8-2.5s | Very Good | $0.016 |

This audio virtualization specification provides comprehensive virtual audio capabilities for KVirtualStage with professional-quality TTS/STT integration and seamless container audio bridging.