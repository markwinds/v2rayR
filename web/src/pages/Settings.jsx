import {useEffect, useState} from 'react';
import {Form, Input, Button, Select, Divider, InputNumber} from 'antd';
import {service, reqSuccessCode} from '../utils/request.jsx';
import {showSuccess} from "../utils/notification.js";

const {Option} = Select;

const Settings = () => {
  const baseUrl = 'settings'

  const [form] = Form.useForm()

  const [currentVersion, setCurrentVersion] = useState("v0.0.0")
  const [latestVersion, setLatestVersion] = useState("v0.0.0")

  async function handleSaveAndRestart(values) {
    let res = await service({
      url: baseUrl + "/save-and-restart",
      method: 'post',
      data: values
    })
    if (res.code !== reqSuccessCode) {
      return
    }
    showSuccess("参数保存成功")
  }

  async function handleExit() {
    let res = await service({
      url: baseUrl + "/stop"
    })
    if (res.code !== reqSuccessCode) {
      return
    }
    showSuccess("程序退出")
  }

  async function handleRestart() {
    let res = await service({
      url: baseUrl + '/restart',
    })
    if (res.code !== reqSuccessCode) {
      return
    }
    showSuccess("重启成功")
  }

  async function handleAutoUpdate() {
    let res = await service({
      url: baseUrl + '/update-client',
    })
    if (res.code !== reqSuccessCode) {
      return
    }
    showSuccess("自动升级成功")
  }

  async function handleResetDefaults() {
    let res = await service({
      url: baseUrl + '/restore-default-param',
    })
    if (res.code !== reqSuccessCode) {
      return
    }
    showSuccess("重置参数成功")
  }

  async function onFinish(values) {
    await handleSaveAndRestart(values)
  }

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

  async function getConfig() {
    let res = await service({
      url: baseUrl + '/get-config',
    })
    if (res.code !== reqSuccessCode) {
      return
    }
    form.setFieldsValue(res.result)
  }

  // 页面刚挂载的时候执行的函数
  useEffect(() => {
    getCurrentVersion()
    getLatestVersion()
    getConfig()
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
        form={form}
        layout="vertical"
        onFinish={onFinish}
        initialValues={{
          logLevel: 'Warning',
          dataDir: 'data',
          webPort: 3333,
          proxy: "",
        }}
      >
        <Form.Item
          name="logLevel"
          label="日志等级"
          rules={[{required: true, message: '请选择日志等级!'}]}
        >
          <Select>
            <Option value="Debug">Debug</Option>
            <Option value="Info">Info</Option>
            <Option value="Warning">Warning</Option>
            <Option value="Error">Error</Option>
          </Select>
        </Form.Item>
        <Form.Item
          name="dataDir"
          label="数据存放目录"
          rules={[{required: true, message: '请输入数据存放目录!'}]}
        >
          <Input/>
        </Form.Item>
        <Form.Item
          name="webPort"
          label="web监听端口"
          rules={[{required: true, message: '请输入web监听端口!'}]}
        >
          <InputNumber/>
        </Form.Item>
        <Form.Item
          name="proxy"
          label="代理"
          rules={[{required: false, message: '软件使用的代理!'}]}
        >
          <Input/>
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit">
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
