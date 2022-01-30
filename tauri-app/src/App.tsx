import React, {useCallback, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import {
  BrowserRouter,
  Routes,
  Route,
} from 'react-router-dom';
import {ChannelList} from './components/ChannelList';
import {ArticleList} from './components/ArticleList';
import styles from './App.module.css';
import "./styles/index.global.css";
import "./App.css";

function useQuery() {
  return new URLSearchParams(window.location.search);
}

enum RouteConfig {
  HOME = '/',
  SETTINGS = '/settings',
  ALL = '/all',
  TODAY = '/today',
  FAVORITE = '/favorite',
  CHANNEL = '/channels/:name',
  ARTICLE = '/channels/:name/articles/:id',
}

function App() {
  const query = useQuery();

  console.log('query', query);

  const
    handleArticleSelect = useCallback((article: any) => {
    }, []);

  return (
    <div className={styles.container}>
      <ChannelList/>
      <BrowserRouter>
        <Routes>
          <Route path={RouteConfig.CHANNEL} element={
            <div className={styles.main} id="appMain">
              <div className={styles.article}>
                {/* <GlobalToolbar /> */}
                <div className={styles.mainView}>
                  <ArticleList
                    title={"asdfasdfsadf"}
                    channelId={"asdf"}
                    onArticleSelect={handleArticleSelect}
                  />
                  {/*<ArticleView article={current} />*/}
                </div>
              </div>
            </div>
          }>
          </Route>
        </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;
