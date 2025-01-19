import React from "react";
import Navbar from "../components/Navbar";

function Home() {
  return (
    <div>
      <Navbar />
      <div style={styles.content}>
        <h1>Welcome to the Blog Project</h1>
      </div>
    </div>
  );
}

const styles = {
  content: {
    paddingTop: "80px", 
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: "calc(100vh - 80px)", 
    textAlign: "center",
  },
};

export default Home;