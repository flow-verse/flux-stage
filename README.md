# flux-stage

Everything that connects Flux to the room.

- **Sync:** Ableton Link tempo/phase; OSC with string-encoded schemas; DMX/Art-Net triggered by tempo shifts and cue points
- **Spatial:** Ambisonic object panner (XYZ + LFO orbits), up to 16 channels
- **Failover:** continuous high-frequency control tone (LifeSine-style) so hardware like the iConnectivity PlayAudio12 can switch to a synchronised backup machine if the primary drops a buffer
- **Input:** MIDI 2.0 / MIDI-CI controller auto-mapping

Rust workspace; real-time paths are allocation- and lock-free.
