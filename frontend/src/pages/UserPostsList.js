import React, { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import Navbar from "../components/Navbar";

function UserPostsList() {
  const [posts, setPosts] = useState([]);

  useEffect(() => {
    fetch("http://localhost:8000/user_posts", {
      headers: {
        Authorization: `Bearer ${localStorage.getItem("token")}`,
      },
    })
      .then((response) => response.json())
      .then((data) => {
        if (data.success) {
          setPosts(data.posts);
        } else {
          console.error("Failed to fetch user posts:", data.message);
        }
      })
      .catch((error) => console.error("Error fetching user posts:", error));
  }, []);

  return (
    <div>
      <Navbar />
      <div style={styles.content}>
        <h1>My Posts</h1>
        {posts.map((post) => (
          <div key={post.id} style={styles.postCard}>
            <Link to={`/posts/${post.id}`} style={styles.postLink}>
              <h2 style={styles.postTitle}>{post.title}</h2>
              <p style={styles.postContent}>
                {post.content.split("\n")[0]}
              </p>
            </Link>
            <Link to={`/posts/${post.id}/edit`} style={styles.editButton}>
              Modify
            </Link>
          </div>
        ))}
      </div>
    </div>
  );
}

const styles = {
  content: {
    padding: "20px",
    paddingTop: "65px",
  },
  editButton: {
    display: "inline-block",
    backgroundColor: "rgb(236, 92, 3)",
    color: "white",
    padding: "10px 20px",
    borderRadius: "5px",
    textDecoration: "none",
    marginTop: "10px",
  },
  postLink: {
    textDecoration: "none",
    color: "inherit",
  },
  postCard: {
    backgroundColor: "#f9f9f9",
    padding: "20px",
    marginBottom: "20px",
    borderRadius: "5px",
    boxShadow: "0 2px 4px rgba(0, 0, 0, 0.1)",
  },
  postTitle: {
    fontSize: "24px",
    marginBottom: "10px",
  },
  postContent: {
    fontSize: "16px",
    color: "#555",
  },
};

export default UserPostsList;