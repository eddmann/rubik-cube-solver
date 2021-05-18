import React, { useState, useRef, useEffect } from 'react';
import { Canvas } from '@react-three/fiber';
import { NoToneMapping } from 'three';
import CameraControls from './CameraControls';
import Actions from './Actions';
import SolutionExplorer from './SolutionExplorer';
import Cube from '../Cube';
import styles from './App.module.scss';
import {
  Solver,
  CubeState,
  Move,
  MoveTransition,
  Solution,
  SOLVED_CUBE,
} from '../constants';

type AppProps = {
  solver: Solver;
};

const doSolve = (
  solver: Solver,
  state: CubeState
): Promise<Solution> => {
  try {
    const solution = solver.solve_cube(state);

    return Promise.resolve([
      ...solution.map(
        (move: Move, idx): MoveTransition => ({
          move,
          startState: solver.apply_cube_moves(
            state,
            solution.slice(0, idx)
          ),
          endState: solver.apply_cube_moves(
            state,
            solution.slice(0, idx + 1)
          ),
        })
      ),
      {
        move: '🎉',
        startState: SOLVED_CUBE,
        endState: SOLVED_CUBE,
      },
    ]);
  } catch (error) {
    return Promise.reject(error);
  }
};

const App = ({ solver }: AppProps) => {
  const cubeRef = useRef<any>();
  const [state, setState] = useState<CubeState>(SOLVED_CUBE);
  const [solution, setSolution] = useState<Solution>([]);
  const [solutionIdx, setSolutionIdx] = useState(0);
  const [isAutoPlay, setAutoPlay] = useState(true);

  useEffect(() => {
    if (!cubeRef.current) return;
    if (!solution[solutionIdx]) return;
    if (!isAutoPlay) return;

    const { move, startState, endState } = solution[solutionIdx];
    if (startState === SOLVED_CUBE) return;

    let isActiveAnimation = true;

    cubeRef.current.rotate(move).then(() => {
      if (!isActiveAnimation) {
        cubeRef.current.reset();
        return;
      }

      setState(endState);

      if (solutionIdx + 1 < solution.length) {
        setSolutionIdx(solutionIdx + 1);
      }
    });

    return () => {
      isActiveAnimation = false;
    };
  }, [solution, solutionIdx, isAutoPlay]);

  const handleRandom = () => {
    setState(solver.rand_cube());
    setSolution([]);
    setSolutionIdx(0);
  };

  const handleSolve = () => {
    doSolve(solver, state)
      .then(solution => {
        setSolution(solution);
        setSolutionIdx(0);
        setAutoPlay(true);
      })
      .catch(error => {
        global.alert(error);
      });
  };

  const handleMoveSelection = (idx: number) => {
    const { startState } = solution[idx];
    setState(startState);
    setSolutionIdx(idx);
  };

  return (
    <div>
      <header>
        <h1>Rubik Cube Solver</h1>
      </header>
      <Actions
        state={state}
        onStateChange={setState}
        onRandom={handleRandom}
        onSolve={handleSolve}
      />
      <Canvas
        onCreated={({ gl }) => {
          gl.toneMapping = NoToneMapping;
        }}
        className={styles.Canvas}
        camera={{ position: [4, 4, 5] }}
        gl={{ antialias: true }}
      >
        <CameraControls />
        <Cube ref={cubeRef} state={state} />
      </Canvas>
      <SolutionExplorer
        solution={solution}
        solutionIdx={solutionIdx}
        isAutoPlay={isAutoPlay}
        onMoveSelection={handleMoveSelection}
        onAutoPlay={setAutoPlay}
      />
    </div>
  );
};

export default App;
