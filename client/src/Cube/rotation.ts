import { Vector3 } from 'three';
import { CubePosition, CubeRotatation } from '../constants';

// prettier-ignore
const ROTATIONS: { [rotation in CubeRotatation]: { positions: CubePosition[], axis: Vector3}} = {
  F: { positions: ['ULF', 'URF', 'DLF', 'DRF'], axis: new Vector3(0, 0, 1) },
  B: { positions: ['URB', 'ULB', 'DRB', 'DLB'], axis: new Vector3(0, 0, 1) },
  R: { positions: ['URF', 'URB', 'DRF', 'DRB'], axis: new Vector3(1, 0, 0) },
  L: { positions: ['ULB', 'ULF', 'DLB', 'DLF'], axis: new Vector3(1, 0, 0) },
  U: { positions: ['ULB', 'URB', 'ULF', 'URF'], axis: new Vector3(0, 1, 0) },
  D: { positions: ['DLF', 'DRF', 'DLB', 'DRB'], axis: new Vector3(0, 1, 0) },
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
  const direction = extra === "'" ? 1 : -1;
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
