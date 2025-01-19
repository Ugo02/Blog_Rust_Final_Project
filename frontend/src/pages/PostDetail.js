import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import Navbar from "../components/Navbar";

function PostDetail() {
  const { id } = useParams();
  const [post, setPost] = useState(null);

  useEffect(() => {
    fetch(`http://localhost:8000/posts/${id}`)
      .then((response) => response.json())
      .then((data) => {
        if (data.success) {
          setPost(data.post); 
        } else {
          console.error("Failed to fetch post:", data.message);
        }
      })
      .catch((error) => console.error("Error fetching post:", error));
  }, [id]);

  if (!post) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <Navbar />
      <div style={styles.content}>
        <h1 style={styles.postTitle}>{post.title}</h1>
        <p style={styles.postContent}>{post.content}</p>
      </div>
    </div>
  );
}

const styles = {
  content: {
    padding: "20px",
    paddingTop: "65px",
  },
  postTitle: {
    fontSize: "32px",
    marginBottom: "20px",
  },
  postContent: {
    fontSize: "18px",
    lineHeight: "1.6",
    whiteSpace: "pre-line",
  },
};

export default PostDetail;