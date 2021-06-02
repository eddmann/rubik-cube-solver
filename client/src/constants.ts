import { Vector3 } from 'three';

export type CubeState = string;

export type CubeRotatation = 'F' | 'B' | 'R' | 'L' | 'U' | 'D';

export type CubePosition =
  | 'DLB'
  | 'DLF'
  | 'ULB'
  | 'ULF'
  | 'DRB'
  | 'DRF'
  | 'URB'
  | 'URF'
  | 'UR'
  | 'UF'
  | 'UL'
  | 'UB'
  | 'DR'
  | 'DF'
  | 'DL'
  | 'DB'
  | 'FR'
  | 'FL'
  | 'BL'
  | 'BR'
  | 'U'
  | 'D'
  | 'R'
  | 'L'
  | 'F'
  | 'B';

export const CUBE_FACELET_COLOURS = {
  W: '#f7f5f5',
  O: 'orange',
  G: 'green',
  R: 'red',
  Y: 'yellow',
  B: 'blue',
};

export type CubeFaceletColour = keyof typeof CUBE_FACELET_COLOURS;

export type Move = string;

export type MoveTransition = {
  move: Move;
  startState: CubeState;
  endState: CubeState;
};

export type Solution = MoveTransition[];

export type Solver = typeof import('wasm-rubik-cube-solver');

export const SOLVED_CUBE =
  'WWWWWWWWWRRRRRRRRRGGGGGGGGGYYYYYYYYYOOOOOOOOOBBBBBBBBB';

export const CUBIE_POSITIONS: {
  [position in CubePosition]: {
    position: Vector3;
    facelets: [
      number | undefined,
      number | undefined,
      number | undefined
    ];
  };
} = {
  DLB: {
    position: new Vector3(-1, -1, -1),
    facelets: [33, 42, 53],
  },
  DLF: {
    position: new Vector3(-1, -1, 1),
    facelets: [27, 44, 24],
  },
  ULB: {
    position: new Vector3(-1, 1, -1),
    facelets: [0, 36, 47],
  },
  ULF: { position: new Vector3(-1, 1, 1), facelets: [6, 38, 18] },
  DRB: {
    position: new Vector3(1, -1, -1),
    facelets: [35, 17, 51],
  },
  DRF: { position: new Vector3(1, -1, 1), facelets: [29, 15, 26] },
  URB: { position: new Vector3(1, 1, -1), facelets: [2, 11, 45] },
  URF: { position: new Vector3(1, 1, 1), facelets: [8, 9, 20] },
  UR: {
    position: new Vector3(1, 1, 0),
    facelets: [5, 10, undefined],
  },
  UF: {
    position: new Vector3(0, 1, 1),
    facelets: [7, undefined, 19],
  },
  UL: {
    position: new Vector3(-1, 1, 0),
    facelets: [3, 37, undefined],
  },
  UB: {
    position: new Vector3(0, 1, -1),
    facelets: [1, undefined, 46],
  },
  DR: {
    position: new Vector3(1, -1, 0),
    facelets: [32, 16, undefined],
  },
  DF: {
    position: new Vector3(0, -1, 1),
    facelets: [28, undefined, 25],
  },
  DL: {
    position: new Vector3(-1, -1, 0),
    facelets: [30, 43, undefined],
  },
  DB: {
    position: new Vector3(0, -1, -1),
    facelets: [34, undefined, 52],
  },
  FR: {
    position: new Vector3(1, 0, 1),
    facelets: [undefined, 12, 23],
  },
  FL: {
    position: new Vector3(-1, 0, 1),
    facelets: [undefined, 41, 21],
  },
  BL: {
    position: new Vector3(-1, 0, -1),
    facelets: [undefined, 39, 50],
  },
  BR: {
    position: new Vector3(1, 0, -1),
    facelets: [undefined, 14, 48],
  },
  U: {
    position: new Vector3(0, 1, 0),
    facelets: [4, undefined, undefined],
  },
  D: {
    position: new Vector3(0, -1, 0),
    facelets: [31, undefined, undefined],
  },
  L: {
    position: new Vector3(-1, 0, 0),
    facelets: [undefined, 40, undefined],
  },
  R: {
    position: new Vector3(1, 0, 0),
    facelets: [undefined, 13, undefined],
  },
  F: {
    position: new Vector3(0, 0, 1),
    facelets: [undefined, undefined, 22],
  },
  B: {
    position: new Vector3(0, 0, -1),
    facelets: [undefined, undefined, 49],
  },
};
