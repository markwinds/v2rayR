import axios from 'axios';
import {notification, message, Spin} from 'antd';
import {LoadingOutlined} from '@ant-design/icons';
import {createRoot} from 'react-dom/client';

let loadingCount = 0;

// 创建一个 Spin 容器
let spinContainer = document.createElement('div');
let tmpRoot;

const showSpin = () => {
  document.body.appendChild(spinContainer);
  tmpRoot = createRoot(spinContainer);

  tmpRoot.render(<Spin size="large"
                       spinning={true}
                       fullscreen
                       indicator={<LoadingOutlined spin/>}
                       delay={200} // 只有请求时间超过500ms时，才会触发显示加载框
    // tip="正在加载......"
  />);
};

const hideSpin = () => {
  tmpRoot.unmount();
  document.body.removeChild(spinContainer);
};


const showLoading = () => {
  if (loadingCount === 0) {
    showSpin()
  }
  loadingCount++;
};

const hideLoading = () => {
  loadingCount--;
  if (loadingCount === 0) {
    hideSpin()
  }
};

// 防抖函数 应用：查找框，在数据变化间隔超过500ms后发起搜索
// const debounce = (func, wait = 300) => {
//   let timeout;
//   return (...args) => {
//     clearTimeout(timeout);
//     timeout = setTimeout(() => func(...args), wait);
//   };
// };

export const service = axios.create({
  baseURL: import.meta.env.VITE_BASE_API || '/',
  timeout: 10000, // 请求超时时间
});

service.interceptors.request.use(
  (config) => {
    showLoading();
    return config;
  },
  (error) => {
    hideLoading();
    return Promise.reject(error);
  }
);

service.interceptors.response.use(
  (response) => {
    hideLoading();
    return response.data;
  },
  (error) => {
    hideLoading();
    if (error.response) {
      const {status, data} = error.response;
      if (status === 400) {
        notification.error({message: '请求错误', description: data.message});
      } else if (status === 401) {
        notification.error({message: '未授权', description: '请登录后再试'});
      } else if (status === 403) {
        notification.error({message: '拒绝访问', description: '您没有权限进行此操作'});
      } else if (status === 404) {
        notification.error({message: '请求地址出错', description: '未找到请求的资源'});
      } else if (status === 500) {
        notification.error({message: '服务器内部错误', description: '请稍后重试'});
      } else {
        notification.error({message: '请求失败', description: data.message || '请稍后重试'});
      }
    } else {
      notification.error({message: '网络错误', description: '无法连接到服务器'});
    }
    return Promise.reject(error);
  }
);

export default {service};
