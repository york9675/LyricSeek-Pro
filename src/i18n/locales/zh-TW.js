export default {
  common: {
    save: '儲存',
    enabled: '啟用',
    disabled: '停用',
    close: '關閉',
    cancel: '取消'
  },
  chooseDirectory: {
    title: '選擇資料夾',
    addDirectory: '新增資料夾',
    continue: '繼續'
  },
  library: {
    listHeaders: {
      track: '歌曲',
      duration: '時長',
      lyrics: '歌詞',
      album: '專輯',
      artist: '歌手'
    },
    tabs: {
      tracks: '歌曲',
      albums: '專輯',
      artists: '藝術家',
      search: '搜尋'
    },
    header: {
      preparing: '準備中',
      downloading: '下載中',
      downloadedSummary: '已下載 {downloaded}/{total}',
      downloadAllLyrics: '下載全部歌詞',
      refreshLibrary: '重新整理資料庫',
      manageDirectories: '管理資料夾',
      settings: '設定',
      about: '關於'
    },
    loading: {
      initializingLibrary: '初始化資料庫中...',
      filesScanned: '已掃描 {scanned}/{total} 個檔案',
      loadingLibrary: '載入資料庫中...'
    },
    track: {
      instrumental: '純音樂',
      synced: '逐字',
      plain: '純文字'
    },
    errors: {
      initializeUnknown: '初始化資料庫時發生未知錯誤。錯誤：{error}'
    }
  },
  searchBar: {
    auto: '自動',
    searchPlaceholder: '搜尋'
  },
  myLrclib: {
    searchPlaceholder: '輸入歌曲名稱、專輯或歌手來搜尋歌詞...',
    toggleAdvanced: '切換進階搜尋',
    search: '搜尋',
    artistOptional: '歌手（選填）',
    albumOptional: '專輯（選填）',
    artist: '歌手',
    album: '專輯',
    provider: '來源',
    providerAuto: '自動（依偏好順序）',
    lrclibTagOptional: 'LRCLIB 標籤（選填）',
    tag: '標籤'
  },
  selectStrategy: {
    title: '選擇策略',
    createLrcTitle: '建立獨立 LRC 檔案',
    createLrcDescription: '將在音樂檔案相同位置建立同名 LRC 歌詞檔。',
    embedTitle: '把歌詞嵌入音樂檔案',
    embedDescription: '歌詞會透過 SYLT 標籤（mp3）或 LYRICS 標籤（flac）寫入檔案，播放器支援有限。',
    skipTracks: '略過已有歌詞的歌曲',
    confirm: '確認'
  },
  flagLyrics: {
    confirmQuestion: '是否要檢舉歌曲 {track} 的歌詞？',
    explainLabel: '請說明你要檢舉這段歌詞的原因：',
    explainPlaceholder: '請輸入檢舉原因...',
    inProgressMessage: '正在檢舉歌曲 {track} 的歌詞...',
    requestChallenge: '請求挑戰中...',
    solveChallenge: '解題中...',
    flagLyrics: '送出檢舉中...',
    confirm: '確認檢舉',
    flagging: '檢舉中',
    success: '歌詞已成功檢舉！'
  },
  publishPlainText: {
    fixProblems: '發佈前請先修正以下問題',
    line: '行號',
    severity: '嚴重程度',
    message: '訊息',
    error: '錯誤',
    confirmQuestion: '是否要將歌曲 {track} 的非同步歌詞發佈到目前的 LRCLIB 服務？',
    inProgressMessage: '正在發佈歌曲 {track} 的非同步歌詞...',
    requestChallenge: '請求挑戰中...',
    solveChallenge: '解題中...',
    publishUnsyncedLyrics: '發佈非同步歌詞中...',
    publishNow: '立即發佈',
    publishing: '發佈中',
    success: '非同步歌詞已成功發佈！搜尋結果可能需要最多 24 小時才會顯示。'
  },
  config: {
    title: '設定',
    groups: {
      common: '一般',
      externalLinks: '外部連結',
      experimental: '實驗功能'
    },
    downloadLyricsFor: {
      label: '歌詞下載範圍',
      allSongs: '下載所有歌曲的歌詞',
      noSynced: '只下載沒有逐字歌詞的歌曲',
      noPlainOrSynced: '只下載沒有純文字或逐字歌詞的歌曲'
    },
    searchSettings: {
      label: '搜尋設定',
      showLineCount: '在搜尋選單顯示歌詞檔行數',
      preferSynced: '優先使用逐字歌詞（必要時回退為純文字歌詞）'
    },
    language: {
      label: '語言',
      system: '跟隨系統語言',
      en: 'English',
      zhTw: '繁體中文'
    },
    themeMode: {
      label: '主題模式',
      auto: '自動',
      light: '淺色',
      dark: '深色'
    },
    lrclibInstance: 'LRCLIB 服務位址',
    providersOrder: {
      label: '歌詞來源順序',
      description: '下載歌詞時會依照由上到下的順序嘗試來源。',
      moveUp: '向上移動來源',
      moveDown: '向下移動來源'
    },
    externalLinks: {
      lastfm: '在播放器列顯示 Last.fm 連結（標題、歌手、專輯）'
    },
    experimental: {
      embedLyrics: '可行時嘗試把歌詞嵌入歌曲檔案',
      warning: '此選項可能會破壞你的歌曲檔案，啟用前請先備份資料庫。'
    },
    actions: {
      refreshLibrary: '重新整理資料庫以套用新變更...',
      manageDirectories: '新增或移除掃描資料夾...'
    },
    providers: {
      lrclib: {
        label: 'LRCLIB',
        description: '來自 LRCLIB 的逐字與純文字歌詞'
      },
      netease: {
        label: 'NetEase',
        description: '來自網易雲音樂的歌詞'
      },
      simpmusic: {
        label: 'SimpMusic',
        description: '來自 SimpMusic 社群的歌詞'
      },
      genius: {
        label: 'Genius',
        description: '來自 Genius 的純文字歌詞'
      }
    }
  },
  about: {
    title: '關於',
    project: '專案',
    description: '支援多來源的桌面歌詞工具，適用於你的離線音樂資料庫。',
    modifiedFromPrefix: 'LyricSeek Pro 修改自',
    licenseSummary: '授權條款（tranxuanthang/lrcget）',
    update: '更新',
    repository: '專案倉庫',
    currentVersion: '目前版本',
    newestVersion: '最新版本',
    updateAvailable: '有可用的新版本。',
    viewReleasePage: '查看發行頁面',
    latestVersion: '你目前使用的是最新版本。',
    updateCheckFailed: '無法從 GitHub 檢查更新。',
    checkingUpdate: '正在從 GitHub 檢查最新發行版本...',
    developer: '開發者',
    contact: '聯絡方式',
    support: '贊助支持'
  }
}
