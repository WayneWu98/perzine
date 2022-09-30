import { getPreferThemeMode, ThemeMode, setPreferTheme } from '@/theme';
import { inject, InjectionKey, onMounted, provide, watch } from 'vue';

const KEY = Symbol() as InjectionKey<{
  themeMode: ThemeMode;
  toggleThemeMode: Function;
  setThemeMode: (mode: ThemeMode) => void;
}>;

export const privideThemeMode = () => {
  let themeMode = $ref<ThemeMode>(getPreferThemeMode());
  const setThemeMode = (mode: ThemeMode) => (themeMode = setPreferTheme(mode));
  const toggleThemeMode = () => {
    switch (themeMode) {
      case ThemeMode.Auto:
        setThemeMode(ThemeMode.Light);
        break;
      case ThemeMode.Light:
        setThemeMode(ThemeMode.Dark);
        break;
      case ThemeMode.Dark:
        setThemeMode(ThemeMode.Auto);
    }
  };
  const updateRootClass = () => {
    console.log('updateRootClass');
    if (getPreferThemeMode() === ThemeMode.Dark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  };
  watch(themeMode, updateRootClass, { immediate: true });
  provide(KEY, { themeMode, toggleThemeMode, setThemeMode });
  onMounted(() => {
    const media = window.matchMedia('(prefers-color-scheme: dark)');
    if (typeof media.addEventListener === 'function') {
      media.addEventListener('change', updateRootClass);
    } else if (typeof media.addListener === 'function') {
      media.addListener(updateRootClass);
    }
  });
};

export default () => {
  const themeMode = inject(KEY);
};
