import {Layout} from 'antd';
import Sidebar from '../components/Sidebar';
import Settings from './Settings.jsx';
import {HashRouter as Router, Route, Routes, Navigate} from 'react-router-dom';

const {Content} = Layout;

const Home = () => (
  <Router>
    <Layout style={{minHeight: '100vh'}}>
      <Sidebar/>
      <Layout style={{padding: '0 24px 24px'}}>
        <Content
          style={{
            padding: 24,
            margin: 0,
            minHeight: 280,
          }}
        >
          <Routes>
            {/*将/重定向到主页*/}
            <Route path="/" element={<Navigate to="/settings" replace/>}/>
            <Route path="/settings" element={<Settings/>}/>
          </Routes>
        </Content>
      </Layout>
    </Layout>
  </Router>
);

export default Home;
