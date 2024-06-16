import {Layout, Menu} from 'antd';
import {SettingOutlined} from '@ant-design/icons';
import {useNavigate} from 'react-router-dom';

const {Sider} = Layout;

const items = [
  {
    key: 'settings',
    label: '设置',
    icon: <SettingOutlined/>,
  },
]


const Sidebar = () => {
  const navigate = useNavigate();
  const onClickMenu = (item) => {
    navigate(item.key);
  }

  return (
    <Sider width={200} className="site-layout-background">
      <Menu
        onClick={onClickMenu}
        mode="inline"
        defaultSelectedKeys={['settings']}
        style={{height: '100%', borderRight: 0}}
        items={items}
      >
      </Menu>
    </Sider>
  )
};

export default Sidebar;
