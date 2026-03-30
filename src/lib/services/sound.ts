export type SoundName = 'click' | 'toggle' | 'add' | 'delete' | 'error' | 'save' | 'undo' | 'modeSwitch' | 'search';

let ctx: AudioContext | null = null;
let enabled = true;

function getCtx(): AudioContext {
  if (!ctx) ctx = new AudioContext();
  return ctx;
}

function playTone(freq: number, type: OscillatorType, durationMs: number, gainVal = 0.15): void {
  const ac = getCtx();
  const osc = ac.createOscillator();
  const g = ac.createGain();
  osc.connect(g);
  g.connect(ac.destination);
  osc.type = type;
  osc.frequency.setValueAtTime(freq, ac.currentTime);
  g.gain.setValueAtTime(gainVal, ac.currentTime);
  g.gain.exponentialRampToValueAtTime(0.001, ac.currentTime + durationMs / 1000);
  osc.start(ac.currentTime);
  osc.stop(ac.currentTime + durationMs / 1000);
}

function playSweep(startFreq: number, endFreq: number, type: OscillatorType, durationMs: number, gainVal = 0.15): void {
  const ac = getCtx();
  const osc = ac.createOscillator();
  const g = ac.createGain();
  osc.connect(g);
  g.connect(ac.destination);
  osc.type = type;
  osc.frequency.setValueAtTime(startFreq, ac.currentTime);
  osc.frequency.linearRampToValueAtTime(endFreq, ac.currentTime + durationMs / 1000);
  g.gain.setValueAtTime(gainVal, ac.currentTime);
  g.gain.exponentialRampToValueAtTime(0.001, ac.currentTime + durationMs / 1000);
  osc.start(ac.currentTime);
  osc.stop(ac.currentTime + durationMs / 1000);
}

export function playSound(name: SoundName): void {
  if (!enabled) return;
  try {
    const ac = getCtx();
    if (ac.state === 'suspended') ac.resume();
    switch (name) {
      case 'click':
        playTone(800, 'square', 50, 0.1);
        break;
      case 'toggle':
        playSweep(440, 880, 'sine', 80, 0.12);
        break;
      case 'add':
        playTone(523, 'sine', 80, 0.12);
        setTimeout(() => playTone(659, 'sine', 100, 0.12), 80);
        break;
      case 'delete':
        playSweep(880, 220, 'sawtooth', 120, 0.1);
        break;
      case 'error':
        playTone(220, 'square', 200, 0.15);
        break;
      case 'save':
        playTone(523, 'sine', 80, 0.1);
        setTimeout(() => playTone(659, 'sine', 80, 0.1), 80);
        setTimeout(() => playTone(784, 'sine', 150, 0.1), 160);
        break;
      case 'undo':
        playSweep(659, 523, 'sine', 100, 0.12);
        break;
      case 'modeSwitch':
        playTone(440, 'square', 60, 0.08);
        break;
      case 'search':
        playTone(660, 'sine', 60, 0.08);
        break;
    }
  } catch {
    // AudioContext may be blocked in some environments
  }
}

export function setSoundEnabled(val: boolean): void {
  enabled = val;
}
