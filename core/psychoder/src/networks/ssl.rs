


/*

https://github.com/wildonion/uniXerr/blob/master/core/recognizer/helper_board

NOTE - feature vectoers learning and representation or extracting semantic attributes using dimensionality reduction algorithms like GA, TSNE, PCA and VAE 
        to compare them with the new unseen input vectors using their distance based on a treshhold between the feature vector representation of the 
        unseen input and the training input embedding vectors model to understand the similarity and abnormality in verification tasks and the 
        category that the new input must be in in classification tasks. 


➔ few train input   -> feature extractor -> input feature vectors for each training input
➔ unseen test input -> feature extractor -> if | unseen feature vectors - each training feature vectors| > treshhold :: abnormality else unseen input belongs to the smallest distance class

*/

pub mod transformers;
pub mod gan;
pub mod vae;
