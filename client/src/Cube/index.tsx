import React, {
  useRef,
  useLayoutEffect,
  useImperativeHandle,
  forwardRef,
} from 'react';
import { useFrame } from '@react-three/fiber';
import { chunk } from 'lodash';
import Cubie from './Cubie';
import animateRotation, { RotationAnimationStep } from './rotation';
import {
  CubePosition,
  CubeFaceletColour,
  CubeState,
  CUBIE_POSITIONS,
  CUBE_FACELET_COLOURS,
  CUBE_STATE_LOOKUP,
} from '../constants';

type CubeProps = {
  state: CubeState;
} & JSX.IntrinsicElements['group'];

type CubeHandle = {
  reset: () => void;
  rotate: (move: string) => Promise<void>;
};

const toCubieColours = (state: CubeState) =>
  chunk(
    [...state].reduce((faces: string[], face, idx) => {
      faces[CUBE_STATE_LOOKUP[idx]] =
        CUBE_FACELET_COLOURS[face as CubeFaceletColour];
      return faces;
    }, []),
    3
  );

const Cube = forwardRef<CubeHandle, CubeProps>(({ state }, ref) => {
  const cubiesRef = useRef<THREE.Mesh[]>([]);
  const rotationRef = useRef<
    | [doStep: RotationAnimationStep, onCompletion: () => void]
    | undefined
  >();

  const stateWithPadding: CubeState = state.padEnd(24, 'W');

  useFrame(() => {
    if (!rotationRef.current) return;

    const [doStep, onCompletion] = rotationRef.current;

    if (!doStep()) {
      onCompletion();
      rotationRef.current = undefined;
    }
  });

  const rotate = (move: string) =>
    new Promise<void>(resolve => {
      rotationRef.current = [
        animateRotation(cubiesRef.current, move, 1),
        resolve,
      ];
    });

  const reset = () => {
    cubiesRef.current.forEach(cubie => {
      cubie.position.copy(cubie.userData.position);
      cubie.rotation.set(0, 0, 0);
    });
  };

  useLayoutEffect(reset, [stateWithPadding]);

  useImperativeHandle(ref, () => ({ reset, rotate }));

  const cubieColours = toCubieColours(stateWithPadding);

  return (
    <group ref={ref}>
      {Object.entries(CUBIE_POSITIONS).map(
        ([name, position], idx) => {
          const [yColour, xColour, zColour] = cubieColours[idx];

          return (
            <Cubie
              key={name}
              ref={el => (cubiesRef.current[idx] = el as THREE.Mesh)}
              name={name as CubePosition}
              position={position}
              yColour={yColour}
              xColour={xColour}
              zColour={zColour}
            />
          );
        }
      )}
    </group>
  );
});

export default Cube;
