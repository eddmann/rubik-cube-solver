import React, { useMemo, useRef } from 'react';
import { Shape, DoubleSide, Vector3 } from 'three';

const eps = 0.00001;

const createShape = (
  width: number,
  height: number,
  radius0: number
): Shape => {
  const shape = new Shape();
  const radius = radius0 - eps;

  shape.absarc(eps, eps, eps, -Math.PI / 2, -Math.PI, true);
  shape.absarc(
    eps,
    height - radius * 2,
    eps,
    Math.PI,
    Math.PI / 2,
    true
  );
  shape.absarc(
    width - radius * 2,
    height - radius * 2,
    eps,
    Math.PI / 2,
    0,
    true
  );
  shape.absarc(width - radius * 2, eps, eps, 0, -Math.PI / 2, true);

  return shape;
};

type FaceletAxis = 'x' | 'y' | 'z';

type FaceletProps = {
  width?: number;
  height?: number;
  radius?: number;
  axis: FaceletAxis;
  colour: string;
  inverse: boolean;
};

const calcPosition = (
  axis: FaceletAxis,
  inverse: boolean
): { rotX: number; rotY: number; position: Vector3 } => {
  switch (axis) {
    case 'y':
      return {
        rotX: 0,
        rotY: (inverse ? -1 : 1) * (Math.PI / 2),
        position: new Vector3(inverse ? -0.5 : 0.5, 0, 0),
      };
    case 'x':
      return {
        rotX: (inverse ? 1 : -1) * (Math.PI / 2),
        rotY: 0,
        position: new Vector3(0, inverse ? -0.5 : 0.5, 0),
      };
    case 'z':
      return {
        rotX: 0,
        rotY: Math.PI,
        position: new Vector3(0, 0, inverse ? -0.5 : 0.5),
      };
  }
};

const Facelet = ({
  width = 0.88,
  height = 0.88,
  radius = 0,
  axis,
  inverse,
  colour,
}: FaceletProps) => {
  const meshRef = useRef<THREE.Mesh>();
  const geometryRef = useRef<THREE.ShapeBufferGeometry>();

  const shape = useMemo(() => createShape(width, height, radius), [
    width,
    height,
    radius,
  ]);
  const { rotX, rotY, position } = calcPosition(axis, inverse);

  React.useLayoutEffect(() => {
    meshRef.current?.rotateX(rotX);
    meshRef.current?.rotateY(rotY);
    geometryRef.current?.center();
  }, [rotY, rotX]);

  return (
    <mesh ref={meshRef} position={position}>
      <meshBasicMaterial
        color={colour}
        side={DoubleSide}
        polygonOffset
        transparent
        polygonOffsetFactor={-1}
        polygonOffsetUnits={-4}
      />
      <shapeBufferGeometry
        ref={geometryRef}
        attach="geometry"
        args={[shape, 5]}
      />
    </mesh>
  );
};

export default Facelet;
