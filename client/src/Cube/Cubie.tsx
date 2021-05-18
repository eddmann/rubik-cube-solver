import React, { forwardRef } from 'react';
import { Shape, ExtrudeBufferGeometry, Mesh, Vector3 } from 'three';
import Facelet from './Facelet';
import { CubePosition } from '../constants';

const eps = 0.00001;

const createShape = (
  width: number,
  height: number,
  radius0: number
) => {
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

type RoundBoxProps = {
  args?: [width?: number, height?: number, depth?: number];
  radius?: number;
  smoothness?: number;
} & Omit<JSX.IntrinsicElements['mesh'], 'args'>;

const RoundedBox = forwardRef<Mesh, RoundBoxProps>(
  function RoundedBox(
    {
      args: [width = 1, height = 1, depth = 1] = [],
      radius = 0.05,
      smoothness = 4,
      children,
      ...rest
    },
    ref
  ) {
    const shape = React.useMemo(
      () => createShape(width, height, radius),
      [width, height, radius]
    );
    const params = React.useMemo(
      () => ({
        depth: depth - radius * 2,
        bevelEnabled: true,
        bevelSegments: smoothness * 2,
        steps: 1,
        bevelSize: radius - eps,
        bevelThickness: radius,
        curveSegments: smoothness,
      }),
      [depth, radius, smoothness]
    );
    const geomRef = React.useRef<ExtrudeBufferGeometry>();

    React.useLayoutEffect(() => {
      if (geomRef.current) {
        geomRef.current.center();
      }
    }, [shape, params]);

    return (
      <mesh ref={ref} {...rest}>
        <extrudeBufferGeometry
          attach="geometry"
          ref={geomRef}
          args={[shape, params]}
        />
        {children}
      </mesh>
    );
  }
);

type CubieProps = {
  name: CubePosition;
  position: Vector3;
  yColour: string;
  xColour: string;
  zColour: string;
};

const Cubie = forwardRef<Mesh, CubieProps>(
  ({ name, position, yColour, xColour, zColour }, ref) => {
    return (
      <RoundedBox
        ref={ref}
        args={[1, 1, 1]}
        radius={0.08}
        smoothness={5}
        position={position}
        userData={{ name, position }}
      >
        <meshBasicMaterial color={0x000000} />
        <Facelet axis="y" inverse={position.x < 0} colour={yColour} />
        <Facelet axis="x" inverse={position.y < 0} colour={xColour} />
        <Facelet axis="z" inverse={position.z < 0} colour={zColour} />
      </RoundedBox>
    );
  }
);

export default Cubie;
