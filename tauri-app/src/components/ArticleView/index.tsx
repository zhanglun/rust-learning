import React, { useCallback, useEffect, useRef, useState } from "react";
import Dayjs from "dayjs";
// @ts-ignore
import Mercury from "@postlight/mercury-parser";
import { Icon } from "../Icon";
import styles from "./view.module.css";
import { getFavico } from "../../helpers/parseXML";

type ArticleViewProps = {
  article: any | null;
};

function createMarkup(html: string) {
  return { __html: html };
}

export const ArticleView = (props: ArticleViewProps): JSX.Element => {
  const { article } = props;
  const containerRef = useRef<HTMLDivElement>(null);
  const helpBarRef = useRef<HTMLDivElement>(null);
  const [pageContent, setPageContent] = useState("");

  const resetScrollTop = () => {
    if (containerRef.current !== null) {
      containerRef.current.scroll(0, 0);
    }
  };

  const openInBrowser = () => {};

  function favoriteIt() {}

  const renderPlaceholder = () => {
    return "";
  };

  const renderDetail = () => {
    if (!article) {
      return null;
    }

    const { feedUrl, link } = article;
    const ico = getFavico(feedUrl);

    return (
      <div className={styles.inner} ref={containerRef}>
        <div className={styles.main}>
          <div className={styles.helpBar} ref={helpBarRef}>
            <div className={styles.menu}>
              <Icon
                customClass={`${styles.menuIcon}`}
                name="done"
                onClick={favoriteIt}
              />
              <Icon
                customClass={`${styles.menuIcon}`}
                name="radio_button_unchecked"
                onClick={favoriteIt}
              />
              <Icon
                customClass={`${styles.menuIcon}`}
                name="favorite"
                onClick={favoriteIt}
              />
              <a target="_blank" rel="noreferrer" href={link}>
                <Icon
                  customClass={`${styles.menuIcon}`}
                  name="link"
                  onClick={openInBrowser}
                />
              </a>
            </div>
          </div>
          <div className={styles.header}>
            <div className={styles.title}>{article.title}</div>
            <div className={styles.meta}>
              <span className={styles.time}>
                {Dayjs().format("YYYY-MM-DD HH:mm")}
              </span>
              <span className={styles.author}>{article.author}</span>
              <span className={styles.channelInfo}>
                <img src={ico} alt="" />
                {article.channelTitle}
              </span>
            </div>
          </div>
          <div className={styles.body}>
            <div
              className={styles.content}
              // eslint-disable-next-line react/no-danger
              dangerouslySetInnerHTML={createMarkup(pageContent)}
            />
          </div>
        </div>
      </div>
    );
  };

  useEffect(() => {
    resetScrollTop();

    if (article) {
      const content = (article.content || article.description || "").replace(
        /<a[^>]+>/gi,
        (a: string) => {
          if (!/\starget\s*=/gi.test(a)) {
            return a.replace(/^<a\s/, '<a target="_blank"');
          }

          return a;
        }
      );

      setPageContent(content);
    }
  }, [article]);

  useEffect(() => {
    if (!containerRef || !containerRef.current) {
      return;
    }

    const handleScroll = () => {
      if (
        containerRef.current &&
        helpBarRef.current &&
        containerRef.current?.scrollTop > 300
      ) {
        console.log("111");
      }
    };

    containerRef.current.addEventListener("scroll", handleScroll);

    return () => {
      if (containerRef.current) {
        containerRef.current.removeEventListener("scroll", handleScroll);
      }
    };
  }, []);

  return (
    // eslint-disable-next-line jsx-a11y/click-events-have-key-events,jsx-a11y/no-static-element-interactions
    <div className={styles.container}>
      {/* {loading && <Loading />} */}
      {article ? renderDetail() : renderPlaceholder()}
    </div>
  );
};
