import React, { useRef } from 'react';
import {
  useFrame,
  useThree,
  extend,
  ReactThreeFiber,
} from '@react-three/fiber';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';

extend({ OrbitControls });

declare global {
  namespace JSX {
    interface IntrinsicElements {
      orbitControls: ReactThreeFiber.Object3DNode<
        OrbitControls,
        typeof OrbitControls
      >;
    }
  }
}

const CameraControls = () => {
  const { camera, gl } = useThree();

  const controls = useRef<OrbitControls>();
  useFrame(() => controls.current?.update());

  return (
    <orbitControls
      ref={controls}
      args={[camera, gl.domElement]}
      enableDamping={true}
      dampingFactor={0.25}
      enableZoom={true}
      enableKeys={false}
      enablePan={false}
      minDistance={4}
      maxDistance={4}
    />
  );
};

export default CameraControls;
