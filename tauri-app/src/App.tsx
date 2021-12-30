import React, { useEffect } from "react";
import { invoke } from '@tauri-apps/api/tauri';
import logo from "./logo.svg";
import tauriCircles from "./tauri.svg";
import tauriWord from "./wordmark.svg";
import "./App.css";

const url = 'http://feed.appinn.com';

function App() {


  const handleClick = async (n: number) => {
    const a = await invoke(`fetch_feed${n}`, { url });
    console.log('a===<', a);
  };

  return (
    <div className="App">
      <header className="App-header">
        <div className="inline-logo">
          <img src={tauriCircles} className="App-logo rotate" alt="logo" />
          <img src={tauriWord} className="App-logo smaller" alt="logo" />
        </div>
        <a
          className="App-link"
          href="https://tauri.studio"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn Tauri
        </a>
        <img src={logo} className="App-logo rotate" alt="logo" />
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        <p>
          <button onClick={() => handleClick(1)}>fetch feed</button>
          <button onClick={() => handleClick(2)}>fetch feed 2</button>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
      </header>
    </div>
  );
}

export default App;
