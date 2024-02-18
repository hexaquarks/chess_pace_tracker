import { ToastAlert, ToastAlertProps } from './ToastAlert';

interface ToastAlertContainerProps {
    toasts: ToastAlertProps[];
    removeToast: (id: number) => void;
}

const ToastAlertContainer: React.FC<ToastAlertContainerProps> = ({ toasts, removeToast }) => {
  return (
    <div className="fixed top-0 right-0 p-4 space-y-2" style={{ zIndex: 1000 }}>
      {toasts.map((toast) => (
        <ToastAlert key={toast.id} {...toast} removeToast={removeToast} />
      ))}
    </div>
  );
};

export default ToastAlertContainer;
