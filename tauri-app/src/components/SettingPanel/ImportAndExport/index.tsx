import React, {useCallback, useRef, useState} from 'react';
import styles from '../settingpanel.module.css';
import {db, Channel as ChannelModel} from '../../../db';

export const ImportAndExport = (props: any) => {
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [file, setFile] = useState<File>();
  const [importedList, setImportedList] = useState<ChannelModel[]>([]);

  const uploadOPMLFile = () => {
    if (fileInputRef && fileInputRef.current) {
      fileInputRef.current.click();
    }
  };

  const parserOPML = (source: string): ChannelModel[] => {
    const parser = new DOMParser();
    const resultDOM = parser.parseFromString(source, 'application/xml');
    const $outlines = resultDOM.querySelectorAll('outline[xmlUrl]');

    return Array.from($outlines)
      .map(($item: Element) => {
        return {
          title: $item.getAttribute('title') || '',
          link: $item.getAttribute('htmlUrl') || '',
          feedUrl: $item.getAttribute('xmlUrl') || '',
        };
      })
      .filter((item) => item.title && item.feedUrl && item.link);
  };

  const importFromOPML = () => {
    console.log(importedList);
    db.channels.bulkAdd(importedList).then((lastkey) => {
      console.log('lastKey: ', lastkey);
    });
  };

  const handleFileChange = (e: any) => {
    setFile(e.target.files[0]);

    const reader = new FileReader();

    reader.onload = () => {
      const xmlString = reader.result as string;
      const list = parserOPML(xmlString);

      setImportedList(list);
    };

    reader.readAsText(e.target.files[0]);
  };

  const exportToOPML = () => {
  };

  return (
    <div className={styles.panel}>
      <div className={styles.panelHeader}>
        <h1 className={styles.panelTitle}>导入</h1>
        <p className={styles.description}>从别处导入您的订阅源</p>
      </div>
      <div className={styles.panelBody}>
        <div className={styles.section}>
          <div className={styles.options}>OPML 导入</div>
          <span>{file && file.name}</span>
          <input
            className={styles.hideFileInput}
            ref={fileInputRef}
            type="file"
            accept=".opml,.xml"
            onChange={(e) => {
              handleFileChange(e);
            }}
          />
          <button onClick={uploadOPMLFile}>浏览</button>
          <button onClick={importFromOPML}>
            导入
          </button>
        </div>
      </div>
      <div className={styles.panelHeader}>
        <h1 className={styles.panelTitle}>导出</h1>
        <p className={styles.description}>
          你可以导出订阅源以便在其他阅读器中使用
        </p>
      </div>
      <div className={styles.panelBody}>
        <div className={styles.section}>
          <div className={styles.options}>OPML 导出</div>
          <button onClick={exportToOPML}>
            导出
          </button>
        </div>
      </div>
    </div>
  );
};
