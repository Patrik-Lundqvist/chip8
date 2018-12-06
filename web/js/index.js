import {Chip8} from '../rs/Cargo.toml';
import {roms, fetchRom} from './roms';
import {setupKeypad} from './keypad';
import {createDisplay} from './display';
import {createUI} from './ui';

let currentRomIndex = 0;
let isStopped = true;
let isLoading = false;
let display;
let ui;

const updateScreen = () => {
  const pixels = emu.get_contents();

  display.drawPixels(pixels);
  ui.updateDebugInfo(
    emu.get_program_counter(),
    emu.get_i_register(),
    emu.get_v_register()
  );
};

const executeCycle = () => {
  emu.execute_cycle();
  updateScreen();
};

const executeNextOpCode = () => {
  emu.execute_next_op_code();
  updateScreen();
};

const fps = 60;
var then = Date.now();
var interval = 1000/fps;
const update = () => {
  window.requestAnimationFrame(update);

  const now = Date.now();
  const delta = now - then;

  if (delta > interval) {
      then = now - (delta % interval);

      if (!isStopped) {
        executeCycle();
      }
  }

};

const stop = () => {
  isStopped = true;
};
const start = () => {
  isStopped = false;
};

const step = () => {
  executeNextOpCode();
};

const getCurrentRom = () => {
  return roms[currentRomIndex];
};

const selectNextGame = () => {
  const nextIndex = currentRomIndex + 1;
  selectRomIndex(nextIndex > roms.length - 1 ? 0 : nextIndex);
};

const selectPrevGame = () => {
  const nextIndex = currentRomIndex - 1;
  selectRomIndex(nextIndex < 0 ? roms.length - 1 : nextIndex);
};

const selectRomIndex = romIndex => {
  if (isLoading) {
    return;
  }
  isLoading = true;
  emu.reset();
  currentRomIndex = romIndex;
  const rom = roms[romIndex];
  ui.setGameLabel(rom.name);
  fetchRom(rom).then(res => {
    emu.load_content(res.value);
    start();
    ui.updateControls(rom.keyMappings);
    isLoading = false;
  });
};

const emu = new Chip8();
display = createDisplay();
ui = createUI(stop, step, selectNextGame, selectPrevGame);
setupKeypad(
  () => getCurrentRom().keyMappings,
  key => emu.press_key(key),
  key => emu.release_key(key)
);
selectRomIndex(0);
window.requestAnimationFrame(update);
