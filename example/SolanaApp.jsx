const App = () => {
  const [message, setMessage] = React.useState('Hello Solana, Web3 React is here!');

  const handleClick = () => {
    setMessage(message === 'Hello Solana, Web3 React is here!' 
      ? 'Solana is the future!' 
      : 'Hello Solana, Web3 React is here!');
  };

  return (
    <div style={{
      fontFamily: 'Arial, sans-serif',
      textAlign: 'center',
      padding: '20px',
      backgroundColor: '#f0f0f0',
      minHeight: '100vh'
    }}>
      <h1 style={{
        background: 'linear-gradient(45deg, #ff6b6b, #4ecdc4)',
        WebkitBackgroundClip: 'text',
        color: 'transparent',
        fontSize: '48px',
        marginBottom: '20px'
      }}>
        {message}
      </h1>
      <p style={{
        fontSize: '24px',
        color: '#333',
        margin: '10px 0'
      }}>
        Welcome to a decentralized React adventure!
      </p>
      <button
        onClick={handleClick}
        style={{
          padding: '10px 20px',
          fontSize: '18px',
          color: 'white',
          backgroundColor: '#ff6b6b',
          border: 'none',
          borderRadius: '5px',
          cursor: 'pointer',
          transition: 'background-color 0.3s'
        }}
        onMouseOver={(e) => e.target.style.backgroundColor = '#ff8787'}
        onMouseOut={(e) => e.target.style.backgroundColor = '#ff6b6b'}
      >
        Click Me!
      </button>
    </div>
  );
};
