export enum ThemeMode {
  Light = 'light',
  Dark = 'dark',
  Auto = 'auto',
}

export const setPreferTheme = (mode: ThemeMode) => {
  localStorage.setItem('theme', mode);
  return mode;
};

export const getPreferThemeMode = () => {
  const storeMode = localStorage.getItem('theme');

  if (storeMode === ThemeMode.Auto) {
    return getSystemThemeMode();
  }

  if (storeMode === ThemeMode.Dark) {
    return ThemeMode.Dark;
  }

  return ThemeMode.Light;
};

export const getSystemThemeMode = () => {
  if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return ThemeMode.Dark;
  }

  return ThemeMode.Light;
};
