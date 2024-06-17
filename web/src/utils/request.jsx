import axios from 'axios';
import {notification, message, Spin} from 'antd';
import ReactDOM from 'react-dom';

let loadingCount = 0;
let loadingMessage = null;

// 创建一个 Spin 容器
let spinContainer = document.createElement('div');

const showSpin = () => {
  spinContainer = document.createElement('div');
  document.body.appendChild(spinContainer);
  ReactDOM.render(<Spin size="large" spinning={true} fullscreen/>, spinContainer);
};

const hideSpin = () => {
  if (spinContainer) {
    ReactDOM.unmountComponentAtNode(spinContainer);
    document.body.removeChild(spinContainer);
  }
};


const showLoading = () => {
  if (loadingCount === 0) {
    loadingMessage = message.loading({content: '正在加载...', duration: 0});
    showSpin()
  }
  loadingCount++;
};

const hideLoading = () => {
  loadingCount--;
  if (loadingCount === 0 && loadingMessage) {
    loadingMessage();
    loadingMessage = null;
    hideSpin()
  }
};

const debounce = (func, wait = 300) => {
  let timeout;
  return (...args) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => func(...args), wait);
  };
};

const service = axios.create({
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

export const debouncedService = {
  get: debounce((url, config) => service.get(url, config)),
  post: debounce((url, data, config) => service.post(url, data, config)),
  put: debounce((url, data, config) => service.put(url, data, config)),
  delete: debounce((url, config) => service.delete(url, config)),
};

export default debouncedService;
