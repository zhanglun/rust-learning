import React from 'react';
import { MainHeader } from '../MainHeader';
import styles from './settingpanel.module.css';

function SettingPanel() {
  return (
    <div className={styles.container}>
      <MainHeader title="设置" />
      <div className={styles.panelContainer}>
      </div>
    </div>
  );
}

export { SettingPanel };
