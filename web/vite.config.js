import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react-swc'
import dotenv from 'dotenv'

// 载入环境变量
dotenv.config()

// https://vitejs.dev/config/
export default defineConfig(() => {
  return {
    base: '/web', // 和部署有关系 url访问时带的base路径
    define: {
      'process.env': {} // 定义process.env变量，防止后续提示未定义
    },
    server: {
      // 本地调试时 监听的ip和端口
      host: process.env.VITE_LISTEN_IP,
      port: process.env.VITE_LISTEN_PORT,
      proxy: {
        // 将请求的代理 解决跨域等问题
        [process.env.VITE_BASE_API]: {
          target: `${process.env.VITE_HOST}`,
          changeOrigin: true,
        },
      },
    },
    plugins: [react()],
  }
})
