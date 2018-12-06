import PONG2 from '../roms/PONG2.ch8';
import BRIX from '../roms/BRIX.ch8';
import TETRIS from '../roms/TETRIS.ch8';
import UFO from '../roms/UFO.ch8';

export const roms = [
  {
    name: 'Ufo',
    file: UFO,
    keyMappings: [
      {emuKey: 0x4, keyCode: 81, description: 'Q - Shoot left'},
      {emuKey: 0x5, keyCode: 87, description: 'W - Shoot up'},
      {emuKey: 0x6, keyCode: 69, description: 'E - Shoot right'},
    ]
  },
  {
    name: 'Pong',
    file: PONG2,
    keyMappings: [
      {emuKey: 0x1, keyCode: 65, description: 'A - Left paddle up'},
      {emuKey: 0x4, keyCode: 90, description: 'Z - Left paddle down'},
      {emuKey: 0xc, keyCode: 75, description: 'K - Right paddle up'},
      {emuKey: 0xd, keyCode: 77, description: 'M - Right paddle down'}
    ]
  },
  {
    name: 'Brix',
    file: BRIX,
    keyMappings: [
      {emuKey: 0x4, keyCode: 37, description: 'Arrow Left - Move left'},
      {emuKey: 0x6, keyCode: 39, description: 'Arrow Right - Move right'},
    ]
  },
  {
    name: 'Tetris',
    file: TETRIS,
    keyMappings: [
      {emuKey: 0x5, keyCode: 37, description: 'Arrow Left - Move left'},
      {emuKey: 0x6, keyCode: 39, description: 'Arrow Right - Move right'},
      {emuKey: 0x7, keyCode: 40, description: 'Arrow Down - Drop'},
      {emuKey: 0x4, keyCode: 32, description: 'Space - Rotate'},
    ]
  },
];

export const fetchRom = rom => {
  return fetch(rom.file).then(res => {
    if (!res.ok) throw new Error(`Could not fetch rom`);

    return res.body
      .getReader()
      .read()
      .then(result => {
        return result;
      });
  });
};
