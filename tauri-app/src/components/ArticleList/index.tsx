/* eslint-disable jsx-a11y/no-static-element-interactions */
/* eslint-disable jsx-a11y/click-events-have-key-events */
import React, {
  useCallback,
  useEffect,
  useRef,
  useState,
} from 'react';
import {invoke} from "@tauri-apps/api/tauri";
// @ts-ignores
import { Dropdown } from '@douyinfe/semi-ui';
import { Icon } from '../Icon';
import { ArticleItem } from '../ArticleItem';
import { Loading } from '../Loading';

import styles from './articlelist.module.css';

type ListFilter = {
  all?: boolean;
  unread?: boolean;
  read?: boolean;
};

type ArticleListProps = {
  channelId: string | null;
  title: string | null;
  onArticleSelect: (article: any) => void;
};

export const ArticleList = (props: ArticleListProps): JSX.Element => {
  const { channelId, title } = props;
  const [loading, setLoading] = useState(true);
  const [articleList, setArticleList] = useState<any[]>([]);
  const articleListRef = useRef<HTMLDivElement>(null);
  const [listFilter, setListFilter] = useState<ListFilter>({
    unread: true,
  });
  const [syncing, setSyncing] = useState(false);

  const resetScrollTop = () => {
    if (articleListRef.current !== null) {
      articleListRef.current.scroll(0, 0);
    }
  };

  const handleArticleSelect = (article: any) => {
    if (props.onArticleSelect) {
      props.onArticleSelect(article);
    }
  };

  const renderList = useCallback((): JSX.Element[] => {
    return articleList.map((article: any) => {
      return (
        <ArticleItem
          article={article}
          key={article.id}
          onSelect={handleArticleSelect}
        />
      );
    });
  }, [articleList]);

  const initial = async () => {
    // await invoke('fetch_feed', {url: channel.url}).then((res) => {
    //   if (typeof res === "string") {
    //     console.log(JSON.parse(res));
    //     // const { items } = res;
    //   }
    // });
  };

  /**
   * 判断是否需要同步
   * @param channel 频道信息
   */
  const checkSyncStatus = (channel: any  | null) => {

  };

  const syncArticles = () => {
  };

  const handleRefresh = () => {
    syncArticles();
  };

  const showAll = () => {
  };

  const showUnread = () => {
  };

  const showRead = () => {
  };

  const markAllRead = () => {
  };

  useEffect(() => {
    resetScrollTop();
  }, []);

  useEffect(() => {
    resetScrollTop();

    initial()
  }, []);

  return (
    <div className={styles.container} ref={articleListRef}>
      <div className={styles.header}>
        <div className={styles.title}>{title}</div>
        <div className={styles.menu}>
          <Icon
            customClass={styles.menuIcon}
            name="checklist"
            onClick={markAllRead}
          />
          <Icon
            customClass={styles.menuIcon}
            name="refresh"
            onClick={handleRefresh}
          />
          <Dropdown
            clickToHide
            trigger="click"
            position="bottomRight"
            render={
              <Dropdown.Menu>
                <Dropdown.Item active={listFilter.unread}>
                  <span
                    className={`${listFilter.unread && styles.active}`}
                    onClick={showUnread}
                  >
                    未读文章
                  </span>
                </Dropdown.Item>
                <Dropdown.Item active={listFilter.read}>
                  <span
                    className={`${listFilter.read && styles.active}`}
                    onClick={showRead}
                  >
                    已读文章
                  </span>
                </Dropdown.Item>
                <Dropdown.Item active={listFilter.all}>
                  <span
                    className={`${listFilter.all && styles.active}`}
                    onClick={showAll}
                  >
                    全部文章
                  </span>
                </Dropdown.Item>
              </Dropdown.Menu>
            }
          >
            <span>
              <Icon customClass={styles.menuIcon} name="filter_alt" />
            </span>
          </Dropdown>
        </div>
      </div>
      <div className={styles.inner}>
        {syncing && <div className={styles.syncingBar}>同步中</div>}
        {loading ? (
          <Loading />
        ) : (
          <ul className={styles.list}>{renderList()}</ul>
        )}
      </div>
    </div>
  );
};
