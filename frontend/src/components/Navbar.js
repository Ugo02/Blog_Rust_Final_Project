import React from "react";
import { Link, useNavigate } from "react-router-dom";

function Navbar() {
  const navigate = useNavigate();
  const isLoggedIn = !!localStorage.getItem("token");

  const handleLogout = () => {
    localStorage.removeItem("token");
    navigate("/login");
  };

  return (
    <nav style={styles.navbar}>
      <div style={styles.leftSection}>
        <Link to="/" style={styles.logo}>Blog Project</Link>
      </div>
      <div style={styles.rightSection}>
        {!isLoggedIn && (
          <>
            <Link to="/register" style={styles.button}>Sign Up</Link>
            <Link to="/login" style={styles.button}>Log In</Link>
          </>
        )}
        {isLoggedIn && (
          <>
            <Link to="/posts" style={styles.button}>Posts</Link>
            <Link to="/user_posts" style={styles.button}>My Posts</Link> 
            <Link to="/create_post" style={styles.button}>+ Post</Link>
            <button onClick={handleLogout} style={styles.button}>
              Logout
            </button>
          </>
        )}
      </div>
    </nav>
  );
}

const styles = {
  navbar: {
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
    backgroundColor: "rgb(236, 92, 3)",
    padding: "10px 20px",
    position: "fixed",
    top: 0,
    left: 0,
    right: 0,
    zIndex: 1000,
  },
  leftSection: {
    flex: 1,
  },
  rightSection: {
    display: "flex",
    gap: "10px",
  },
  logo: {
    color: "white",
    fontSize: "24px",
    fontWeight: "bold",
    textDecoration: "none",
  },
  button: {
    backgroundColor: "white",
    color: "rgb(236, 92, 3)",
    padding: "10px 20px",
    borderRadius: "5px",
    textDecoration: "none",
    fontWeight: "bold",
    border: "none",
    cursor: "pointer",
  },
};

export default Navbar;