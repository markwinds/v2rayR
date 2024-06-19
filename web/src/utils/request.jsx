import axios from 'axios';
import {Spin} from 'antd';
import {LoadingOutlined} from '@ant-design/icons';
import {createRoot} from 'react-dom/client';
import {showError} from "./notification.js";

let loadingCount = 0;
export let reqSuccessCode = 0;

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

    if (response.config.responseType === 'arraybuffer') {
      if (!response.headers.hasOwnProperty('err-code')) {
        // 没有错误信息 直接返回二进制数据
        return response
      }
      // 有错误信息 将arraybuffer转为json
      const arrayBuffer = response.data
      const dataView = new DataView(arrayBuffer)
      const decoder = new TextDecoder('utf-8')
      const jsonString = decoder.decode(dataView)
      response.data = JSON.parse(jsonString)
    }
    if (response.data.code === reqSuccessCode) {
      return response.data
    }

    console.log("data", response.data)
    showError(response.data.zh_msg === '' ? '未知错误' : response.data.zh_msg)

    return response.data
  },
  (error) => {
    hideLoading();
    if (error.response) {
      const {status, data} = error.response;
      if (status === 400) {
        showError(status + '：请求错误')
      } else if (status === 401) {
        showError(status + '：未授权')
      } else if (status === 403) {
        showError(status + '：拒绝访问')
      } else if (status === 404) {
        showError(status + '：请求地址出错')
      } else if (status === 500) {
        showError(status + '：服务器内部错误')
      } else {
        showError(status + '：请求失败')
      }
    } else {
      showError("请求失败，请检查网络")
    }

    return Promise.reject(error);
  }
);

export default {service, reqSuccessCode};
