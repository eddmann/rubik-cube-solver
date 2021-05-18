import React from 'react';
import styles from './App.module.scss';
import { Solution } from '../constants';

type SolutionExplorerProps = {
  solution: Solution;
  solutionIdx: number;
  isAutoPlay: boolean;
  onMoveSelection: (idx: number) => void;
  onAutoPlay: (enabled: boolean) => void;
};

const SolutionExplorer = ({
  solution,
  solutionIdx,
  isAutoPlay,
  onMoveSelection,
  onAutoPlay,
}: SolutionExplorerProps) => {
  if (solution.length === 0) {
    return null;
  }

  return (
    <div className={styles.Solution}>
      {solution.map(({ move }, idx) => (
        <span
          key={idx}
          className={
            solutionIdx === idx ? styles.MoveActive : styles.Move
          }
          onClick={() => onMoveSelection(idx)}
        >
          {move}
        </span>
      ))}
      <label>
        <input
          type="checkbox"
          onChange={() => onAutoPlay(!isAutoPlay)}
          checked={isAutoPlay}
        />
        Auto Play
      </label>
    </div>
  );
};

export default SolutionExplorer;
