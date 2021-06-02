import React, {
  useRef,
  useLayoutEffect,
  useImperativeHandle,
  forwardRef,
} from 'react';
import { useFrame } from '@react-three/fiber';
import Cubie from './Cubie';
import animateRotation, { RotationAnimationStep } from './rotation';
import {
  CubePosition,
  CubeFaceletColour,
  CubeState,
  CUBIE_POSITIONS,
  CUBE_FACELET_COLOURS,
} from '../constants';

type CubeProps = {
  state: CubeState;
} & JSX.IntrinsicElements['group'];

type CubeHandle = {
  reset: () => void;
  rotate: (move: string) => Promise<void>;
};

const Cube = forwardRef<CubeHandle, CubeProps>(({ state }, ref) => {
  const cubiesRef = useRef<THREE.Mesh[]>([]);
  const rotationRef =
    useRef<
      | [doStep: RotationAnimationStep, onCompletion: () => void]
      | undefined
    >();

  const stateWithPadding: CubeState = state.padEnd(54, 'W');

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

  return (
    <group ref={ref}>
      {Object.entries(CUBIE_POSITIONS).map(
        ([name, { position, facelets }], idx) => {
          const [xColour, yColour, zColour] = facelets.map(idx =>
            idx !== undefined
              ? CUBE_FACELET_COLOURS[
                  stateWithPadding[idx] as CubeFaceletColour
                ]
              : undefined
          );

          return (
            <Cubie
              key={name}
              ref={el => (cubiesRef.current[idx] = el as THREE.Mesh)}
              name={name as CubePosition}
              position={position}
              xColour={xColour}
              yColour={yColour}
              zColour={zColour}
            />
          );
        }
      )}
    </group>
  );
});

export default Cube;
