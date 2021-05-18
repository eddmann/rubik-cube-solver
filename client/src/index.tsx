import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';

import './new-1.1.3.css';

import('wasm-rubik-cube-solver').then(solver => {
  ReactDOM.render(
    <React.StrictMode>
      <App solver={solver} />
    </React.StrictMode>,
    document.getElementById('root')
  );
});
