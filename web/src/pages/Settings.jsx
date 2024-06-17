// src/components/Settings.jsx
import React, {useState} from 'react';
import {Form, Input, Button, Select, Divider, notification} from 'antd';
import {debouncedService as axios} from '../utils/request.jsx';

const {Option} = Select;

const Settings = () => {
  const [logLevel, setLogLevel] = useState('Info');
  const [dataDirectory, setDataDirectory] = useState('');
  const [webPort, setWebPort] = useState('');

  const currentVersion = '1.0.0'; // 当前软件版本
  const latestVersion = '1.1.0';  // 最新客户端版本

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

  const handleExit = () => {
    // 退出程序的逻辑
    axios.post('/api/exit')
      .then(() => {
        notification.warning({message: '程序已退出'});
      })
      .catch((error) => {
        notification.error({message: '退出程序失败'});
      });
  };

  const handleRestart = () => {
    // 重启程序的逻辑
    axios.post('/api/restart')
      .then(() => {
        notification.info({message: '程序已重启'});
      })
      .catch((error) => {
        notification.error({message: '重启程序失败'});
      });
  };

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
