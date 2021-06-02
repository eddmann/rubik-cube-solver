import { Vector3 } from 'three';
import { CubePosition, CubeRotatation } from '../constants';

// prettier-ignore
const ROTATIONS: { [rotation in CubeRotatation]: { positions: CubePosition[], axis: Vector3}} = {
  F: { positions: ['ULF', 'URF', 'DLF', 'DRF', 'UF', 'DF', 'FR', 'FL', 'F'], axis: new Vector3(0, 0, 1) },
  B: { positions: ['URB', 'ULB', 'DRB', 'DLB', 'UB', 'DB', 'BL', 'BR', 'B'], axis: new Vector3(0, 0, 1) },
  R: { positions: ['URF', 'URB', 'DRF', 'DRB', 'UR', 'DR', 'FR', 'BR', 'R'], axis: new Vector3(1, 0, 0) },
  L: { positions: ['ULB', 'ULF', 'DLB', 'DLF', 'UL', 'DL', 'FL', 'BL', 'L'], axis: new Vector3(1, 0, 0) },
  U: { positions: ['ULB', 'URB', 'ULF', 'URF', 'UR', 'UF', 'UL', 'UB', 'U'], axis: new Vector3(0, 1, 0) },
  D: { positions: ['DLF', 'DRF', 'DLB', 'DRB', 'DR', 'DF', 'DL', 'DB', 'D'], axis: new Vector3(0, 1, 0) },
};

export type RotationAnimationStep = () => boolean;

const animateRotation = (
  cubies: any[],
  move: string,
  animationSpeed: number
): RotationAnimationStep => {
  const [rotation, extra] = move.split('', 2) as [
    CubeRotatation,
    "'" | '2' | undefined
  ];
  const { positions, axis } = ROTATIONS[rotation];
  let direction = extra === "'" ? 1 : -1;
  if (['D', 'L', 'B'].includes(rotation)) direction = -direction;
  const quarterTurns = extra === '2' ? 2 : 1;
  const cubiesToRotate = cubies.filter(cubie =>
    positions.includes(cubie.userData.name)
  );
  const stepFactor = 0.05 * quarterTurns * animationSpeed;
  const targetRotation = (quarterTurns * Math.PI) / 2;
  let remaining = targetRotation;

  return () => {
    if (remaining <= 0) {
      return false;
    }

    const theta =
      (1.1 -
        ((2 * remaining - targetRotation) / targetRotation) ** 2) *
      stepFactor;
    remaining -= theta;

    cubiesToRotate.forEach(cubie => {
      cubie.position.applyAxisAngle(axis, theta * direction);
      cubie.rotateOnAxis(axis, theta * direction);
    });

    return true;
  };
};

export default animateRotation;
