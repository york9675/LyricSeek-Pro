export default {
  common: {
    save: 'Save',
    enabled: 'Enabled',
    disabled: 'Disabled',
    close: 'Close',
    cancel: 'Cancel'
  },
  chooseDirectory: {
    title: 'Select directories',
    addDirectory: 'Add new directory',
    continue: 'Continue'
  },
  library: {
    listHeaders: {
      track: 'Track',
      duration: 'Duration',
      lyrics: 'Lyrics',
      album: 'Album',
      artist: 'Artist'
    },
    tabs: {
      tracks: 'Tracks',
      albums: 'Albums',
      artists: 'Artists',
      search: 'Search'
    },
    header: {
      preparing: 'Preparing',
      downloading: 'Downloading',
      downloadedSummary: 'Downloaded {downloaded}/{total}',
      downloadAllLyrics: 'Download all lyrics',
      refreshLibrary: 'Refresh library',
      manageDirectories: 'Manage directories',
      settings: 'Settings',
      about: 'About'
    },
    loading: {
      initializingLibrary: 'Initializing library...',
      filesScanned: '{scanned}/{total} files scanned',
      loadingLibrary: 'Loading library...'
    },
    track: {
      instrumental: 'Instrumental',
      synced: 'Synced',
      plain: 'Plain'
    },
    errors: {
      initializeUnknown: 'Unknown error happened when initializing the library. Error: {error}'
    }
  },
  searchBar: {
    auto: 'Auto',
    searchPlaceholder: 'Search'
  },
  myLrclib: {
    searchPlaceholder: 'Type a song title, album, or artist to find lyrics...',
    toggleAdvanced: 'Toggle advanced search',
    search: 'Search',
    artistOptional: 'Artist (optional)',
    albumOptional: 'Album (optional)',
    artist: 'Artist',
    album: 'Album',
    provider: 'Provider',
    providerAuto: 'Auto (Preference order)',
    lrclibTagOptional: 'LRCLIB tag (optional)',
    tag: 'Tag'
  },
  selectStrategy: {
    title: 'Select Strategy',
    createLrcTitle: 'Create separate LRC files',
    createLrcDescription: 'LRC lyrics files will be created in the same location with the music files and have the same name.',
    embedTitle: 'Embed lyrics to music files',
    embedDescription: 'Lyrics will be embedded to music files through SYLT tag (mp3) or LYRICS tag (flac). Limited players support.',
    skipTracks: 'Skip tracks that already have lyrics',
    confirm: 'Confirm'
  },
  flagLyrics: {
    confirmQuestion: 'Do you want to flag the lyrics of the song {track}?',
    explainLabel: 'Please explain why you want to flag the lyrics:',
    explainPlaceholder: 'Explain why you want to flag the lyrics...',
    inProgressMessage: 'Flagging the lyrics of the song {track}...',
    requestChallenge: 'Request challenge...',
    solveChallenge: 'Solve challenge...',
    flagLyrics: 'Flag lyrics...',
    confirm: 'Confirm Flag',
    flagging: 'Flagging',
    success: 'The lyrics has been flagged successfully!'
  },
  publishPlainText: {
    fixProblems: 'Please fix the following problem(s) before publishing',
    line: 'Line',
    severity: 'Severity',
    message: 'Message',
    error: 'Error',
    confirmQuestion: 'Do you want to publish your unsynchronized lyrics of the song {track} to your current LRCLIB instance?',
    inProgressMessage: 'Publishing your unsynchronized lyrics of the song {track}...',
    requestChallenge: 'Request challenge...',
    solveChallenge: 'Solve challenge...',
    publishUnsyncedLyrics: 'Publish unsynced lyrics...',
    publishNow: 'Publish Now',
    publishing: 'Publishing',
    success: 'Your unsynced lyrics has been published successfully! It might take up to 24 hours to be visible on the search results.'
  },
  config: {
    title: 'Configuration',
    groups: {
      common: 'Common',
      externalLinks: 'External Links',
      experimental: 'Experimental'
    },
    downloadLyricsFor: {
      label: 'Download lyrics for',
      allSongs: 'Download lyrics for all songs',
      noSynced: 'Download lyrics for songs without synced lyrics',
      noPlainOrSynced: 'Download lyrics for songs without plain or synced lyrics'
    },
    searchSettings: {
      label: 'Search settings',
      showLineCount: 'Show the number of lines a lyric file has in the search menu',
      preferSynced: 'Prefer synced lyrics first across providers (fallback to plain lyrics if needed)'
    },
    language: {
      label: 'Language',
      system: 'Follow system language',
      en: 'English',
      zhTw: '繁體中文'
    },
    themeMode: {
      label: 'Theme mode',
      auto: 'Auto',
      light: 'Light',
      dark: 'Dark'
    },
    lrclibInstance: 'LRCLIB instance',
    providersOrder: {
      label: 'Lyrics provider order',
      description: 'Providers are tried from top to bottom when downloading lyrics.',
      moveUp: 'Move provider up',
      moveDown: 'Move provider down'
    },
    externalLinks: {
      lastfm: 'Enable Last.fm links on player bar (title, artist, album)'
    },
    experimental: {
      embedLyrics: 'Try to embed the lyrics to the track files when possible',
      warning: 'This option could corrupt your track files. Make sure to backup your library before enabling it.'
    },
    actions: {
      refreshLibrary: 'Refresh my library for new changes...',
      manageDirectories: 'Add and remove scanning directories...'
    },
    providers: {
      lrclib: {
        label: 'LRCLIB',
        description: 'Synced and plain lyrics from LRCLIB'
      },
      netease: {
        label: 'NetEase',
        description: 'Lyrics from NetEase Cloud Music'
      },
      simpmusic: {
        label: 'SimpMusic',
        description: 'Community lyrics from SimpMusic'
      },
      genius: {
        label: 'Genius',
        description: 'Plain lyrics from Genius'
      }
    }
  },
  about: {
    title: 'About',
    project: 'Project',
    description: 'Desktop lyrics tool with multi-provider support for your offline music library.',
    modifiedFromPrefix: 'LyricSeek Pro is a modified version of',
    licenseSummary: 'License (tranxuanthang/lrcget)',
    update: 'Update',
    repository: 'Repository',
    currentVersion: 'Current version',
    newestVersion: 'Newest version',
    updateAvailable: 'New update is available.',
    viewReleasePage: 'View release page',
    latestVersion: 'You are using the latest version.',
    updateCheckFailed: 'Failed to check updates from GitHub.',
    checkingUpdate: 'Checking latest release from GitHub...',
    developer: 'Developer',
    contact: 'Contact',
    support: 'Support'
  }
}
