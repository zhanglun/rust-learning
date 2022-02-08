import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import {BrowserRouter, Route, Routes} from "react-router-dom";
import {RouteConfig} from "./config";
import {ArticleContainer} from "./containers/Article";
import {SettingContainer} from "./containers/Setting";

ReactDOM.render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path={"/"} element={<App/>}>
          <Route path={RouteConfig.CHANNEL} element={<ArticleContainer/>}>
          </Route>
        </Route>
        <Route path={RouteConfig.SETTINGS} element={<SettingContainer/>}>
        </Route>
      </Routes>
    </BrowserRouter>
  </React.StrictMode>,
  document.getElementById('root')
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
