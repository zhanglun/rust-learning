import React, { useState, useEffect } from 'react';
// @ts-ignores
import Dayjs from 'dayjs';
import styles from './articleitem.module.css';

import bannerImage1 from './images/pattern-1.png';
import bannerImage2 from './images/pattern-2.png';
import bannerImage3 from './images/pattern-3.png';
import bannerImage4 from './images/pattern-4.png';
import bannerImage5 from './images/pattern-5.png';

const bannerImageList = [
  bannerImage1,
  bannerImage2,
  bannerImage3,
  bannerImage4,
  bannerImage5,
];

export const ArticleItem = (props: any) => {
  const { article, onSelect } = props;
  const [readStatus, setReadStatus] = useState(false);
  const [bannerImage, setBannerImage] = useState('');

  const markAsRead = () => {};

  console.log(article.pubDate);

  const handleClick = (e: any) => {
    if(onSelect) {
      onSelect(article);
    }
  };

  const parseBannerImage = (content: string): string => {
    const banner = bannerImageList[Math.ceil(Math.random() * 4)];

    if (!content) {
      return banner;
    }

    const matchs = content.match(/<img[^>]+src="(\S*)"/);

    return (matchs && matchs[1]) || banner;
  };

  useEffect(() => {
    setBannerImage(parseBannerImage(article.content));
  }, [article]);

  return (
    <li
      className={`${styles.item} ${readStatus && styles.read}`}
      onClick={handleClick}
      aria-hidden="true"
    >
      <div className={styles.header}>
        <div
          className={styles.image}
          style={{ backgroundImage: `url("${bannerImage}")` }}
        />
        <div className={styles.title}>
          <div className={styles.titleText}>{article.title}</div>
        </div>
        {/* <div className={styles.actions}> */}
        {/*  <Icon customClass={styles.icon} name="bookmark_add" /> */}
        {/*  <Icon customClass={styles.icon} name="favorite_border" /> */}
        {/*  <Icon customClass={styles.icon} name="done" onClick={markAsRead} /> */}
        {/*  <Icon customClass={styles.icon} name="launch" onClick={openWebPage} /> */}
        {/* </div> */}
        <div className={styles.date}>
          {Dayjs(article.pubDate).format('YYYY-MM-DD HH:mm')}
        </div>
      </div>
    </li>
  );
};
