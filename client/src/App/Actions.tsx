import React from 'react';
import { CubeState, CUBE_FACELET_COLOURS } from '../constants';
import styles from './App.module.scss';

type ActionsProps = {
  state: CubeState;
  onStateChange: (state: CubeState) => void;
  onRandom: () => void;
  onSolve: () => void;
};

const Actions = ({
  state,
  onStateChange,
  onRandom,
  onSolve,
}: ActionsProps) => {
  const unknownColours = new RegExp(
    `[^${Object.keys(CUBE_FACELET_COLOURS).join('')}]`,
    'g'
  );

  return (
    <div className={styles.Actions}>
      <input
        size={80}
        type="text"
        value={(state.match(/.{1,9}/g) || []).join(' ')}
        onChange={e =>
          onStateChange(
            e.target.value.toUpperCase().replace(unknownColours, '')
          )
        }
      />
      <br />
      <button onClick={onRandom}>Random</button>
      <button onClick={onSolve}>Solve</button>
    </div>
  );
};

export default Actions;
