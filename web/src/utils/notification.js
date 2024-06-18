import {notification} from "antd";

// 过滤短时间内重复的信息
let msgList = []

// 判断是否有重复信息
function hasMsg(msg) {
  return msgList.includes(msg)
}

function pushMsg(msg) {
  msgList.push(msg)
}

function rmMsg(msg) {
  msgList = msgList.filter(item => item !== msg)
}


function showMsg(msg, type, title) {
  if (hasMsg(msg)) {
    return
  }
  pushMsg(msg)

  // 延时显示，保证提示展示在顶层
  setTimeout(() => {
    notification[type]({
      message: title,
      description: msg,
      duration: 4.5,
      placement: "bottomRight",
      showProgress: true,
      onClose: () => {
        rmMsg(msg) // 同一时刻只能展示一个相同的提示
      }
    });
  }, 10)
}

function showInfo(msg) {
  showMsg(msg, 'info', "信息")
}

function showWarning(msg) {
  showMsg(msg, 'warning', "警告")
}

function showError(msg) {
  showMsg(msg, 'error', "错误")
}

function showSuccess(msg) {
  showMsg(msg, 'success', "成功")
}

export {showSuccess, showInfo, showWarning, showError}