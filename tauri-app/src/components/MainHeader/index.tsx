import React from "react";
import { Icon } from "../Icon";
import { requestFeed } from "../../helpers/parseXML";
import { Toast } from "../Toast";
import { db, Article as ArticleModel, Article } from "../../db";

import styles from "./header.module.css";
import { collapseTextChangeRangesAcrossMultipleVersions } from "typescript";

type MainHeaderProps = {
  channelId: string | null;
  feedUrl: string | null;
  title: string | null;
};

export const MainHeader = (props: MainHeaderProps) => {
  const { feedUrl, title } = props;

  const syncArticles = () => {
    feedUrl &&
      requestFeed(feedUrl).then((res) => {
        if (res.channel && res.items) {
          const { items } = res;
          const links = items.map((item: ArticleModel) => item.link);

          db.articles
            .where("link")
            .anyOf(links)
            .toArray()
            .then((exists) => {
              if (exists.length < items.length) {
                const remotes = items.filter((item: Article) => {
                  return !exists.some((exist) => exist.link === item.link);
                });

                db.transaction("rw", db.articles, async () => {
                  db.articles.bulkAdd(remotes);
                }).then(() => {
                  Toast.show({
                    title: "success",
                    content: "Sync Success!",
                  });
                });
              }
            });
        }
      });
  };

  const handleRefresh = () => {
    syncArticles();
  };

  const markAllRead = () => {};

  return (
    <div className={styles.container}>
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
        </div>
      </div>
    </div>
  );
};
