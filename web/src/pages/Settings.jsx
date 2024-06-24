// src/components/Settings.jsx
import React, {useEffect, useReducer, useState} from 'react';
import {Form, Input, Button, Select, Divider, notification} from 'antd';
import {service, service as axios, reqSuccessCode} from '../utils/request.jsx';
import {showSuccess} from "../utils/notification.js";

const {Option} = Select;

const Settings = () => {
  const baseUrl = 'settings'
  const [logLevel, setLogLevel] = useState('');
  const [dataDirectory, setDataDirectory] = useState('');
  const [webPort, setWebPort] = useState(0);

  const [currentVersion, setCurrentVersion] = useState("v0.0.0")
  const [latestVersion, setLatestVersion] = useState("v0.0.0")

  const handleSaveAndRestart = () => {
    // 保存参数并重启的逻辑
    axios.get('/settings/save-and-reset')
      .then((response) => {
        notification.success({message: '保存参数并重启成功'});
      })
      .catch((error) => {
        notification.error({message: '保存参数并重启失败'});
      });
  };

  async function handleExit() {
    let res = await service({
      url: baseUrl + "/stop"
    })
    if (res.code !== reqSuccessCode) {
      return
    }
    showSuccess("程序退出")
  };

  async function handleRestart() {
    let res = await service({
      url: baseUrl + '/restart',
    })
    if (res.code !== reqSuccessCode) {
      return
    }
    showSuccess("重启成功")
  }

  const handleAutoUpdate = () => {
    // 自动更新的逻辑
    axios.post('/api/auto-update')
      .then(() => {
        notification.success({message: '自动更新成功'});
      })
      .catch((error) => {
        notification.error({message: '自动更新失败'});
      });
  };

  const handleResetDefaults = () => {
    // 恢复默认参数的逻辑
    setLogLevel('Info');
    setDataDirectory('');
    setWebPort('');
    notification.success({message: '已恢复默认参数'});
  };

  const onFinish = (values) => {
    console.log('Received values from form: ', values);
  };

  async function getLatestVersion() {
    let res = await service({
      url: baseUrl + '/latest-version',
    })
    if (res.code !== reqSuccessCode) {
      return
    }
    setLatestVersion(res.result)
  }

  async function getCurrentVersion() {
    let res = await service({
      url: baseUrl + '/now-version',
    })
    if (res.code !== reqSuccessCode) {
      return
    }
    setCurrentVersion(res.result)
  }

  // 页面刚挂载的时候执行的函数
  useEffect(() => {
    getCurrentVersion()
    getLatestVersion()
  })

  return (
    <div>
      <h2>软件版本信息</h2>
      <p>当前版本: {currentVersion}</p>
      <p>最新版本: {latestVersion}</p>
      <Button type="default" onClick={handleAutoUpdate} style={{marginTop: '10px', marginBottom: '20px'}}>
        自动更新
      </Button>
      <Divider/>

      <h2>设置项</h2>
      <Form
        name="settings"
        layout="vertical"
        onFinish={onFinish}
        initialValues={{
          logLevel: 'Info',
          dataDirectory: '',
          webPort: '',
        }}
      >
        <Form.Item
          name="logLevel"
          label="日志等级"
          rules={[{required: true, message: '请选择日志等级!'}]}
        >
          <Select value={logLevel} onChange={setLogLevel}>
            <Option value="Debug">Debug</Option>
            <Option value="Info">Info</Option>
            <Option value="Warning">Warning</Option>
            <Option value="Error">Error</Option>
          </Select>
        </Form.Item>
        <Form.Item
          name="dataDirectory"
          label="数据存放目录"
          rules={[{required: true, message: '请输入数据存放目录!'}]}
        >
          <Input value={dataDirectory} onChange={(e) => setDataDirectory(e.target.value)}/>
        </Form.Item>
        <Form.Item
          name="webPort"
          label="web监听端口"
          rules={[{required: true, message: '请输入web监听端口!'}]}
        >
          <Input value={webPort} onChange={(e) => setWebPort(e.target.value)}/>
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit" onClick={handleSaveAndRestart}>
            保存参数并重启
          </Button>
        </Form.Item>
      </Form>
      <Divider/>

      <h2>操作按钮</h2>
      <Button type="default" onClick={handleExit} style={{marginRight: '10px'}}>
        退出程序
      </Button>
      <Button type="default" onClick={handleRestart} style={{marginRight: '10px'}}>
        重启程序
      </Button>
      <Button type="default" onClick={handleResetDefaults}>
        恢复默认参数
      </Button>
    </div>
  );
};

export default Settings;
