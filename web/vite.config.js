import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vitejs.dev/config/
export default defineConfig(()=>{
  return {
    base: 'web', // 和部署有关系 url访问时带的base路径
    plugins: [react()],
  }
})
