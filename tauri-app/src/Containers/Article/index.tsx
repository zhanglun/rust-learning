import React, { useCallback, useState } from 'react';
import { useLocation, useParams } from 'react-router-dom';
import { ArticleList } from '../../components/ArticleList';
// import { Article } from '../../components/Article';
import styles from './index.module.css';
// import { Article } from '../../../infra/types';

function useQuery() {
  return new URLSearchParams(useLocation().search);
}

export const ArticleContainer = (): JSX.Element => {
  // @ts-ignore
  const params: {name: string} = useParams();
  const query = useQuery();
  const [current, setCurrent] = useState<any>(null);

  const handleArticleSelect = useCallback((article: any) => {
    setCurrent(article);
  }, []);

  return (
    <div className={styles.article}>
      {/* <GlobalToolbar /> */}
      <div className={styles.mainView}>
        <ArticleList
          title={params.name}
          channelId={query.get('channelId')}
          feedUrl={query.get('feedUrl')}
          onArticleSelect={handleArticleSelect}
        />
        {/*<Article article={current} />*/}
      </div>
    </div>
  );
};
