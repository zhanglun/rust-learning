import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {
  HashRouter as Router,
} from 'react-router-dom';
import { ChannelList } from './components/ChannelList';
import styles from './App.module.css';
import "./App.css";

const url = "http://feed.appinn.com";

function App() {
  const [res, setRes] = useState<any>("");

  const handleClick = async (n: number) => {
    const a = await invoke(`fetch_feed`, { url });
    console.log("a===<", a);
    setRes(a);
  };

  const request = () => {
    fetch(url)
      .then((res) => res.text())
      .then((res) => {
        setRes(res);
      })
      .catch((err) => {
        setRes(err.message);
      });
  };

  return (
        <div className={styles.container}>
          <p>
            <button onClick={() => handleClick(1)}>fetch feed</button>
            <button onClick={() => request()}>request</button>
          </p>
          <p>{res}</p>
          <Router>
            <ChannelList />
          </Router>
        </div>
  );
}

export default App;
