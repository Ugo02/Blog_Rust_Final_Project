import React, { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import Navbar from "../components/Navbar";

function PostsList() {
  const [posts, setPosts] = useState([]);

  useEffect(() => {
    fetch("http://localhost:8000/posts")
      .then((response) => response.json())
      .then((data) => {
        if (data.success) {
          setPosts(data.posts); 
        } else {
          console.error("Failed to fetch posts:", data.message);
        }
      })
      .catch((error) => console.error("Error fetching posts:", error));
  }, []);

  return (
    <div>
      <Navbar />
      <div style={styles.content}>
        <h1>Posts</h1>
        {posts.map((post) => (
          <Link to={`/posts/${post.id}`} key={post.id} style={styles.postLink}>
            <div style={styles.postCard}>
              <h2 style={styles.postTitle}>{post.title}</h2>
              <p style={styles.postContent}>
                {post.content.split("\n")[0]} 
              </p>
            </div>
          </Link>
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

export default PostsList;