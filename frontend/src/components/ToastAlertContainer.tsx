import React from 'react';
import { ToastAlert, ToastAlertProps } from './ToastAlert';
import { CSSTransition, TransitionGroup } from 'react-transition-group';

interface ToastAlertContainerProps {
    toasts: ToastAlertProps[];
    removeToast: (id: number) => void;
}

const ToastAlertContainer: React.FC<ToastAlertContainerProps> = ({ toasts, removeToast }) => {
  return (
    <div className="fixed top-0 right-0 p-4 space-y-2" style={{ zIndex: 1000 }}>
      <TransitionGroup>
        {toasts.map((toast) => (
          <CSSTransition
            key={toast.id}
            timeout={500}
            classNames="toast"
          >
            <ToastAlert {...toast} removeToast={removeToast} />
          </CSSTransition>
        ))}
      </TransitionGroup>
    </div>
  );
};

export default ToastAlertContainer;