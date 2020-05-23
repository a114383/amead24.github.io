# building off our other image
FROM mc3

LABEL mainter=tony

RUN conda update conda && \
    conda install --quiet --yes ipython && \
    conda clean -tipsy

# each dockerfile can contain a single CMD instruction
# preference is for it to run the designated software
CMD ["ipython"]
