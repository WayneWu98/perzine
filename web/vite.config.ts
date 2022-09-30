import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import legacy from '@vitejs/plugin-legacy';
import { resolve } from 'path';
import WindiCSS from 'vite-plugin-windicss';

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
      service: resolve(__dirname, 'src/service'),
      hooks: resolve(__dirname, 'src/hooks'),
      components: resolve(__dirname, 'src/components'),
      views: resolve(__dirname, 'src/views'),
    },
  },
  build: {
    rollupOptions: {
      input: {
        index: 'index.html',
        admin: 'admin/index.html',
      },
    },
  },
  plugins: [
    vue({
      reactivityTransform: true,
    }),
    legacy({
      targets: ['defaults', 'not IE 11'],
    }),
    WindiCSS(),
  ],
});