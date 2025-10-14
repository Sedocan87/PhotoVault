import React from 'react';

interface ToastProps {
  message: string;
  type: 'success' | 'error';
  onClose: () => void;
}

const Toast: React.FC<ToastProps> = ({ message, type, onClose }) => {
  const bgColor = type === 'success' ? 'bg-green-500' : 'bg-red-500';

  return (
    <div className={`fixed top-4 right-4 ${bgColor} text-white p-4 rounded-lg shadow-lg`}>
      <p>{message}</p>
      <button onClick={onClose} className="absolute top-1 right-2 text-white">
        &times;
      </button>
    </div>
  );
};

export default Toast;